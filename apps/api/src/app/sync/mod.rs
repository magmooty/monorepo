use std::sync::Arc;

use axum::{routing::post, Router};

mod check_sync_availability;

mod test_check_sync_availability;

use check_sync_availability::*;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new().route("/check_sync_availability", post(check_sync_availability))
}
