use serde::Serialize;
use serde::{self, Deserialize};
use specta::Type;

#[derive(Serialize, Type)]
pub enum AppError {
    #[serde(rename = "internal_error")]
    InternalError(String),
}

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum InternetConnectionStatus {
    Connected,
    NotConnected,
}

#[derive(Serialize, Type)]
pub struct InternetConnectionCheckResponse {
    pub connection_status: InternetConnectionStatus,
}

#[tauri::command]
#[specta::specta]
pub async fn check_internet_connection() -> Result<InternetConnectionCheckResponse, AppError> {
    Ok(InternetConnectionCheckResponse {
        connection_status: InternetConnectionStatus::Connected,
    })
}
