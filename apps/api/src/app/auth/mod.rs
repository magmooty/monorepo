use std::sync::Arc;

use axum::{routing::post, Router};

pub mod resend_signin_code;
pub mod send_signin_code;

mod test_send_signin_code;
mod test_resend_signin_code;

pub use resend_signin_code::*;
pub use send_signin_code::*;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/send_signin_code", post(send_signin_code))
        .route("/resend_signin_code", post(resend_signin_code))
}
