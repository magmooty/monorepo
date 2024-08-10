use std::sync::Arc;

use axum::{routing::post, Router};

mod send_signin_code;

mod test_send_signin_code;

use send_signin_code::*;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new().route("/send_signin_code", post(send_signin_code))
}
