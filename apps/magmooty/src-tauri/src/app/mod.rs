use crate::{router, TauriAppContext};
use serde;
use serde::Serialize;
use serde_json::{from_value, Value};
use tauri::State;
pub mod whatsapp;

#[derive(Serialize)]
#[serde(untagged)]
pub enum AppResponse {
    WhatsAppInfoResponse(whatsapp::WhatsAppInfoResponse),
    WhatsAppStartConnectionResponse(whatsapp::WhatsAppStartConnectionResponse),
    WhatsAppSendMessageResponse(whatsapp::WhatsAppSendMessageResponse),
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum AppError {
    InternalError(String),
}

#[tauri::command]
pub fn query(state: State<TauriAppContext>, path: String, body: Value) -> String {
    let application = state.application.as_ref();
    match application.handle_route(path, body) {
        Ok(response) => serde_json::to_string(&response).unwrap(),
        Err(error) => serde_json::to_string(&error).unwrap(),
    }
}

pub fn run() -> router::Router<AppResponse, AppError> {
    let mut application: router::Router<AppResponse, AppError> = router::Router::for_root();
    application.join_router(whatsapp::get_router());
    application
}
