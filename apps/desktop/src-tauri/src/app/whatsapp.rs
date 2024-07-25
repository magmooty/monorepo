use super::AppError;
use serde::Serialize;
use serde::{self, Deserialize};
use specta::Type;
use tokio::task;

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum WhatsAppConnectionStatus {
    SignedIn,
    SignedOut,
    #[serde(rename = "qr_code_generated")]
    QRCodeGenerated,
    #[serde(rename = "whatsapp_library_error")]
    WhatsAppLibraryError,
    #[serde(rename = "target_not_on_whatsapp")]
    TargetNotOnWhatsApp,
}

fn __parse_connection_status(connection_status: String) -> WhatsAppConnectionStatus {
    match connection_status.as_str() {
        "signed_in" => WhatsAppConnectionStatus::SignedIn,
        "signed_out" => WhatsAppConnectionStatus::SignedOut,
        "qr_code_generated" => WhatsAppConnectionStatus::QRCodeGenerated,
        "whatsapp_library_error" => WhatsAppConnectionStatus::WhatsAppLibraryError,
        "target_not_on_whatsapp" => WhatsAppConnectionStatus::TargetNotOnWhatsApp,
        _ => WhatsAppConnectionStatus::WhatsAppLibraryError,
    }
}

#[derive(Serialize, Type)]
pub struct WhatsAppInfoResponse {
    pub connection_status: WhatsAppConnectionStatus,
}

#[tauri::command]
#[specta::specta]
pub async fn whatsapp_get_info() -> Result<WhatsAppInfoResponse, AppError> {
    let wa_response = task::spawn_blocking(move || crate::whatsapp::get_info())
        .await
        .expect("Paniced while communicating with WhatsApp library");

    let connection_status = __parse_connection_status(wa_response.connection_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(WhatsAppInfoResponse { connection_status }),
    }
}

#[derive(Serialize, Type)]
pub struct WhatsAppStartConnectionResponse {
    pub code: String,
    pub connection_status: WhatsAppConnectionStatus,
}

#[tauri::command]
#[specta::specta]
pub async fn whatsapp_start_connection() -> Result<WhatsAppStartConnectionResponse, AppError> {
    let wa_response = task::spawn_blocking(move || crate::whatsapp::start_connection())
        .await
        .expect("Paniced while communicating with WhatsApp library");

    let connection_status = __parse_connection_status(wa_response.connection_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(WhatsAppStartConnectionResponse {
            code: wa_response.code,
            connection_status,
        }),
    }
}

#[derive(Deserialize, Type)]
pub struct SendMessageBody {
    phone_number: String,
    message: String,
}

#[derive(Serialize, Type)]
pub struct WhatsAppSendMessageResponse {
    pub message_status: WhatsAppMessageStatus,
    pub connection_status: WhatsAppConnectionStatus,
}

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "snake_case")]
pub enum WhatsAppMessageStatus {
    Successful,
    Failed,
}

fn __parse_message_status(message_status: String) -> WhatsAppMessageStatus {
    match message_status.as_str() {
        "successful" => WhatsAppMessageStatus::Successful,
        "failed" => WhatsAppMessageStatus::Failed,
        _ => WhatsAppMessageStatus::Failed,
    }
}

#[tauri::command]
#[specta::specta]
pub async fn whatsapp_send_message(
    body: SendMessageBody,
) -> Result<WhatsAppSendMessageResponse, AppError> {
    let wa_response = task::spawn_blocking(move || {
        crate::whatsapp::send_message(body.phone_number, body.message)
    })
    .await
    .expect("Paniced while communicating with WhatsApp library");

    let connection_status = __parse_connection_status(wa_response.connection_status);
    let message_status = __parse_message_status(wa_response.message_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(WhatsAppSendMessageResponse {
            message_status,
            connection_status,
        }),
    }
}
