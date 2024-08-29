use std::sync::Arc;

use admin::AdminSecurityAddon;
use axum::Router;
use serde::Serialize;
use telegram_bot::TelegramClient;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;
use validator::Validate;

use crate::database::Database;

mod admin;
mod auth;
mod common;
mod sync;

use sync::ChunkUploadSchemasAddon;

#[utoipauto(paths = "./apps/api/src/app")]
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Magmooty Central API",
        description = "Handles authorization and offline -> remote syncing"
    ),
    modifiers(&VersionAddon, &AdminSecurityAddon, &ChunkUploadSchemasAddon)
)]
struct ApiDoc;

pub struct VersionAddon;

impl Modify for VersionAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.info.version = env!("CARGO_PKG_VERSION").to_string()
    }
}

#[derive(Serialize, Debug)]
pub struct AppErrorResponse {
    pub error_message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub telegram: Arc<TelegramClient>,
}

impl AppState {
    pub fn new(db: Arc<Database>, telegram: Arc<TelegramClient>) -> Self {
        Self { db, telegram }
    }
}

pub fn create_app_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/admin", admin::get_router())
        .nest("/sync", sync::get_router())
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
}

pub fn validate_payload<T>(payload: T)
where
    T: Validate,
{
    match payload.validate() {
        Ok(validated) => validated,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
