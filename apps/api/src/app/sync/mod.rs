use std::sync::Arc;

use axum::{routing::post, Router};

pub mod check_sync_availability;
pub mod upload_chunk;

mod test_check_sync_availability;
mod test_upload_chunk;

pub use check_sync_availability::*;
pub use upload_chunk::*;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/check_sync_availability", post(check_sync_availability))
        .route("/upload_chunk", post(upload_chunk))
}
