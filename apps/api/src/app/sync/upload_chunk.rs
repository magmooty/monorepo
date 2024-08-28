use std::sync::Arc;

use crate::app::{validate_payload, AppState};
use axum::extract::State;
use axum::{debug_handler, http::StatusCode, Json};
use base64::Engine;
use jsonwebtoken::{self, decode_header, Algorithm, Validation};
use log::{debug, info, warn};
use serde;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::{Datetime, Thing};
use utoipa::ToSchema;
use validator::Validate;

static LOG_TARGET: &str = "Upload chunk";

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SyncEvent {
    record_id: Value,
    event: String,
    content: Value,
    created_at: Datetime,
}

#[derive(Serialize, Deserialize, Validate, Debug, ToSchema)]
pub struct UploadChunkPayload {
    pub signature: String,
    pub chunk: Vec<SyncEvent>,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UploadChunkStatus {
    Accepted,
    CenterNotFound,
    SignatureInvalid,
}

#[derive(Serialize, Debug, ToSchema)]
pub struct UploadChunkResponse {
    status: UploadChunkStatus,
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "Synchronization",
    path = "/sync/upload_chunk",
    request_body = UploadChunkPayload,
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
    Json(payload): Json<UploadChunkPayload>,
) -> (StatusCode, Json<UploadChunkResponse>) {
    validate_payload(&payload);
    debug!(target: LOG_TARGET, "Received chunk for center {:?}", payload);

    (
        StatusCode::OK,
        Json(UploadChunkResponse {
            status: UploadChunkStatus::Accepted,
        }),
    )
}
