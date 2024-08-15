use std::sync::Arc;

use crate::app::{validate_payload, AppState};
use axum::extract::State;
use axum::{debug_handler, http::StatusCode, Json};
use base64::Engine;
use jsonwebtoken::{self, decode_header, Algorithm, Validation};
use log::{info, warn};
use serde;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

static LOG_TARGET: &str = "Check sync availability";

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema)]
pub struct CheckSyncAvailabilityPayload {
    pub center_id: String,
    pub signature: String,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum CheckSyncAvailabilityStatus {
    CenterNotFound,
    CenterSignatureInvalid,
    Available,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct CheckSyncAvailabilityResponse {
    status: CheckSyncAvailabilityStatus,
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "Synchronization",
    path = "/sync/check_sync_availability",
    request_body = CheckSyncAvailabilityPayload,
    responses(
        (status = OK, description = "Available", body = CheckSyncAvailabilityResponse, example = json!({ "status": "available" })),
        (status = UNAUTHORIZED, description = "Invalid or manipulated signature for the center", body = CheckSyncAvailabilityResponse, example = json!({ "status": "center_signature_invalid" })),
        (status = NOT_FOUND, description = "Center not found", body = CheckSyncAvailabilityResponse, example = json!({ "status": "center_not_found" }))
    )
)]
pub async fn check_sync_availability(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CheckSyncAvailabilityPayload>,
) -> (StatusCode, Json<CheckSyncAvailabilityResponse>) {
    validate_payload(&payload);

    info!(target: LOG_TARGET, "Decoding signature header for center {}", payload.center_id);
    match decode_header(&payload.signature) {
        Ok(header) => {
            if header.alg != Algorithm::RS256 {
                warn!(target: LOG_TARGET, "Invalid algorithm for token");
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(CheckSyncAvailabilityResponse {
                        status: CheckSyncAvailabilityStatus::CenterSignatureInvalid,
                    }),
                );
            }

            header
        }
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(CheckSyncAvailabilityResponse {
                    status: CheckSyncAvailabilityStatus::CenterSignatureInvalid,
                }),
            )
        }
    };

    info!(target: LOG_TARGET, "Looking for center {}", payload.center_id);
    let center = match state.db.center.get_center(&payload.center_id).await {
        Some(center) => center,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(CheckSyncAvailabilityResponse {
                    status: CheckSyncAvailabilityStatus::CenterNotFound,
                }),
            )
        }
    };

    info!(target: LOG_TARGET, "Decoding center {} public key", payload.center_id);
    let public_key = match base64::prelude::BASE64_STANDARD.decode(center.public_key) {
        Ok(public_key) => public_key,
        Err(err) => {
            warn!(target: LOG_TARGET, "Error decoding public key: {}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(CheckSyncAvailabilityResponse {
                    status: CheckSyncAvailabilityStatus::CenterSignatureInvalid,
                }),
            );
        }
    };

    let decoding_key = jsonwebtoken::DecodingKey::from_rsa_der(&public_key);

    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_required_spec_claims(&["center_id"]);

    info!(target: LOG_TARGET, "Verifying center {} signature", payload.center_id);
    let token = match jsonwebtoken::decode::<serde_json::Value>(
        &payload.signature,
        &decoding_key,
        &validation,
    ) {
        Ok(token) => token,
        Err(err) => {
            warn!(target: LOG_TARGET, "Error decoding token: {}", err);
            return (
                StatusCode::UNAUTHORIZED,
                Json(CheckSyncAvailabilityResponse {
                    status: CheckSyncAvailabilityStatus::CenterSignatureInvalid,
                }),
            );
        }
    };

    if token.claims["center_id"] != payload.center_id.to_string() {
        warn!(target: LOG_TARGET, "Token center_id {} does not match payload center_id {}", token.claims["center_id"], payload.center_id);
        return (
            StatusCode::UNAUTHORIZED,
            Json(CheckSyncAvailabilityResponse {
                status: CheckSyncAvailabilityStatus::CenterSignatureInvalid,
            }),
        );
    }

    info!(target: LOG_TARGET, "Valid signature for center {}", payload.center_id);
    (
        StatusCode::OK,
        Json(CheckSyncAvailabilityResponse {
            status: CheckSyncAvailabilityStatus::Available,
        }),
    )
}
