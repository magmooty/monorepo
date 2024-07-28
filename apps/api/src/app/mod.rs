use axum::Router;
use serde::Serialize;
use validator::Validate;

mod auth;

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error_message: String,
}

pub fn get_router() -> Router {
    Router::new().nest("/auth", auth::get_router())
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
