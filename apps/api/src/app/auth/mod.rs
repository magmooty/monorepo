use std::sync::Arc;

use axum::{routing::post, Router};

mod resend_signin_code;
mod send_signin_code;

mod test_send_signin_code;
mod test_resend_signin_code;

use resend_signin_code::*;
use send_signin_code::*;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/send_signin_code", post(send_signin_code))
        .route("/resend_signin_code", post(resend_signin_code))
}
