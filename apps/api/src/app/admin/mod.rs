use std::sync::Arc;

use axum::middleware::from_fn;
use axum::{routing::post, Router};

pub mod generate_qr_code;
mod middleware;

use generate_qr_code::*;

use tower::ServiceBuilder;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::Modify;

use super::AppState;

pub struct AdminSecurityAddon;

impl Modify for AdminSecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme(
                    "admin_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        )
    }
}

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
