use std::sync::Arc;

use axum::Router;
use serde::Serialize;
use validator::Validate;

use crate::database::Database;

mod admin;
mod auth;
mod sync;

#[derive(Serialize, Debug)]
pub struct AppErrorResponse {
    pub error_message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
}

impl AppState {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

pub fn create_app_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/admin", admin::get_router())
        .nest("/sync", sync::get_router())
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
