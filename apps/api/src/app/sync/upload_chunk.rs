use std::sync::Arc;

use crate::app::{validate_payload, AppState};
use crate::database::SyncEvent;
use axum::body::Bytes;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::{debug_handler, http::StatusCode, Json};
use base64::Engine;
use log::{debug, info, warn};
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::signature::Verifier;
use rsa::{pkcs1::DecodeRsaPublicKey, sha2::Sha256, RsaPublicKey};
use serde;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

static LOG_TARGET: &str = "Upload chunk";

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema)]
pub struct UploadChunkPayload {
    pub chunk: Vec<SyncEvent>,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UploadChunkStatus {
    Accepted,
    CenterNotFound,
    SignatureInvalid,
    ChunkInvalid,
    MissingHeaders,
    Base64DecodeError,
    PrivateKeyParseError,
    DatabaseUploadError,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct UploadChunkResponse {
    status: UploadChunkStatus,
}

async fn verify_chunk(
    body: &Bytes,
    signature: &String,
    public_key: &String,
) -> Result<(), UploadChunkStatus> {
    let body = body.clone();
    let signature = signature.clone();
    let public_key = public_key.clone();

    #[cfg(any(debug_assertions, test))]
    {
        if signature.eq("debug") {
            debug!(target: LOG_TARGET, "Automatically verifying signature for debug mode");
            return Ok(());
        }
    }

    tokio::task::spawn_blocking(move || {
        let public_key = base64::prelude::BASE64_STANDARD
            .decode(public_key)
            .map_err(|_| UploadChunkStatus::Base64DecodeError)?;

        let signature = base64::prelude::BASE64_STANDARD
            .decode(signature)
            .map_err(|_| UploadChunkStatus::Base64DecodeError)?;

        let signature = Signature::try_from(signature.as_slice())
            .map_err(|_| UploadChunkStatus::SignatureInvalid)?;

        let public_key = RsaPublicKey::from_pkcs1_der(&public_key)
            .map_err(|_| UploadChunkStatus::PrivateKeyParseError)?;

        let verifying_key = VerifyingKey::<Sha256>::new(public_key);

        verifying_key
            .verify(&body, &signature)
            .map_err(|_| UploadChunkStatus::SignatureInvalid)?;

        Ok(())
    })
    .await
    .map_err(|_| UploadChunkStatus::SignatureInvalid)?
}

fn extract_header(
    header_name: &str,
    headers: &HeaderMap,
) -> Result<String, (StatusCode, Json<UploadChunkResponse>)> {
    match headers.get(header_name) {
        Some(value) => match value.to_str() {
            Ok(value) => Ok(value.to_string()),
            Err(_) => {
                warn!(target: LOG_TARGET, "Invalid signature");
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(UploadChunkResponse {
                        status: UploadChunkStatus::MissingHeaders,
                    }),
                ));
            }
        },
        None => {
            warn!(target: LOG_TARGET, "No signature provided");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(UploadChunkResponse {
                    status: UploadChunkStatus::SignatureInvalid,
                }),
            ));
        }
    }
}

fn parse_payload(
    payload: &Bytes,
) -> Result<UploadChunkPayload, (StatusCode, Json<UploadChunkResponse>)> {
    match serde_json::from_slice::<UploadChunkPayload>(&payload) {
        Ok(payload) => Ok(payload),
        Err(err) => {
            warn!(target: LOG_TARGET, "Invalid payload {:?}", err.to_string());
            return Err((
                StatusCode::BAD_REQUEST,
                Json(UploadChunkResponse {
                    status: UploadChunkStatus::ChunkInvalid,
                }),
            ));
        }
    }
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "Synchronization",
    path = "/sync/upload_chunk",
    request_body = UploadChunkPayload,
    params(
        ("Signature", Header, description = "Signature of raw request body"),
        ("Center-ID", Header, description = "Center ID"),
    ),
    responses(
        (status = OK, description = "Chunk uploaded", body = UploadChunkResponse, example = json!({ "status": "accepted" })),
        (status = UNAUTHORIZED, description = "Invalid or manipulated signature", body = UploadChunkResponse, example = json!({ "status": "signature_invalid" })),
        (status = NOT_FOUND, description = "Center not found", body = UploadChunkResponse, example = json!({ "status": "center_not_found" }))
    )
)]
pub async fn upload_chunk(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    payload: Bytes,
) -> (StatusCode, Json<UploadChunkResponse>) {
    debug!(target: LOG_TARGET, "Received chunk");
    debug!(target: LOG_TARGET, "Checking headers");

    let center_id = match extract_header("Center-ID", &headers) {
        Ok(center) => center.to_string(),
        Err(response) => return response,
    };

    let signature = match extract_header("Signature", &headers) {
        Ok(signature) => signature,
        Err(response) => return response,
    };

    debug!(target: LOG_TARGET, "Checking if center {} exists", &center_id);

    let center = match state.db.clone().center.get_center(&center_id).await {
        Some(center) => center,
        None => {
            warn!(target: LOG_TARGET, "Center not found");
            return (
                StatusCode::NOT_FOUND,
                Json(UploadChunkResponse {
                    status: UploadChunkStatus::CenterNotFound,
                }),
            );
        }
    };

    debug!(target: LOG_TARGET, "Checking chunk signature for center {}", &center_id);

    match verify_chunk(&payload, &signature, &center.public_key).await {
        Ok(_) => {}
        Err(response) => {
            warn!(target: LOG_TARGET, "Invalid chunk signature for center {}: {:?}", &center_id, &response);
            return (
                StatusCode::UNAUTHORIZED,
                Json(UploadChunkResponse { status: response }),
            );
        }
    };

    let payload = match parse_payload(&payload) {
        Ok(payload) => payload,
        Err(response) => return response,
    };

    validate_payload(&payload);

    info!(target: LOG_TARGET, "Received chunk of {} for center {}", payload.chunk.len(), &center_id);

    match state
        .db
        .clone()
        .sync
        .insert_sync_events(&center_id, payload.chunk)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(UploadChunkResponse {
                status: UploadChunkStatus::Accepted,
            }),
        ),
        Err(err) => {
            warn!(target: LOG_TARGET, "Error inserting chunk: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadChunkResponse {
                    status: UploadChunkStatus::DatabaseUploadError,
                }),
            )
        }
    }
}
