use axum::middleware::from_fn;
use axum::{routing::post, Router};

mod generate_qr_code;
mod middleware;

use generate_qr_code::generate_whatsapp_qr_code;
use tower::ServiceBuilder;

pub fn get_router() -> Router {
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
