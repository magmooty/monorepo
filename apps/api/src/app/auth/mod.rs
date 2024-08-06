use axum::{routing::post, Router};

mod send_signin_code;

use send_signin_code::*;

pub fn get_router() -> Router {
    Router::new().route("/send_signin_code", post(send_signin_code))
}
