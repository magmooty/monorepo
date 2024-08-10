use std::sync::Arc;

use axum::middleware::from_fn;
use axum::{routing::post, Router};

mod generate_qr_code;
mod middleware;

use generate_qr_code::*;

use tower::ServiceBuilder;

use super::AppState;

pub fn get_router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/whatsapp/generate_qr_code",
            post(generate_whatsapp_qr_code),
        )
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(middleware::jwt_middleware))
                .into_inner(),
        )
}
