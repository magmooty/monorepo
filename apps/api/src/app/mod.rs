use axum::Router;
use serde::Serialize;
use validator::Validate;

mod admin;
mod auth;

#[derive(Serialize, Debug)]
pub struct AppErrorResponse {
    pub error_message: String,
}

pub fn get_router() -> Router {
    Router::new()
        .nest("/auth", auth::get_router())
        .nest("/admin", admin::get_router())
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
