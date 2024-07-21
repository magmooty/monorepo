use crate::router;
use serde::Serialize;
use serde::{self, Deserialize};
use serde_json::Value;

use super::{AppError, AppResponse};

#[derive(Serialize)]
pub struct WhatsAppInfoResponse {
    pub connection_status: WhatsAppConnectionStatus,
}

#[derive(Serialize)]
pub struct WhatsAppStartConnectionResponse {
    pub code: String,
    pub connection_status: WhatsAppConnectionStatus,
}

#[derive(Serialize)]
pub struct WhatsAppSendMessageResponse {
    pub message_status: WhatsAppMessageStatus,
    pub connection_status: WhatsAppConnectionStatus,
}

#[derive(Deserialize, Serialize)]
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

fn parse_connection_status(connection_status: String) -> WhatsAppConnectionStatus {
    match connection_status.as_str() {
        "signed_in" => WhatsAppConnectionStatus::SignedIn,
        "signed_out" => WhatsAppConnectionStatus::SignedOut,
        "qr_code_generated" => WhatsAppConnectionStatus::QRCodeGenerated,
        "whatsapp_library_error" => WhatsAppConnectionStatus::WhatsAppLibraryError,
        "target_not_on_whatsapp" => WhatsAppConnectionStatus::TargetNotOnWhatsApp,
        _ => WhatsAppConnectionStatus::WhatsAppLibraryError,
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WhatsAppMessageStatus {
    Successful,
    Failed,
}

fn parse_message_status(message_status: String) -> WhatsAppMessageStatus {
    match message_status.as_str() {
        "successful" => WhatsAppMessageStatus::Successful,
        "failed" => WhatsAppMessageStatus::Failed,
        _ => WhatsAppMessageStatus::Failed,
    }
}

pub fn get_whatsapp_info(_: Value) -> Result<AppResponse, AppError> {
    let wa_response = crate::whatsapp::get_info();

    let connection_status = parse_connection_status(wa_response.connection_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(AppResponse::WhatsAppInfoResponse(WhatsAppInfoResponse {
            connection_status,
        })),
    }
}

pub fn start_connection(_: Value) -> Result<AppResponse, AppError> {
    let wa_response = crate::whatsapp::start_connection();

    let connection_status = parse_connection_status(wa_response.connection_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(AppResponse::WhatsAppStartConnectionResponse(
            WhatsAppStartConnectionResponse {
                code: wa_response.code,
                connection_status,
            },
        )),
    }
}

#[derive(Deserialize)]
pub struct SendMessageBody {
    phone_number: String,
    message: String,
}

pub fn send_message(body: Value) -> Result<AppResponse, AppError> {
    let SendMessageBody {
        phone_number,
        message,
    }: SendMessageBody = serde_json::from_value(body).unwrap();

    let wa_response = crate::whatsapp::send_message(phone_number, message);

    let connection_status = parse_connection_status(wa_response.connection_status);
    let message_status = parse_message_status(wa_response.message_status);

    match connection_status {
        WhatsAppConnectionStatus::WhatsAppLibraryError => {
            Err(AppError::InternalError(wa_response.error_message))
        }
        _ => Ok(AppResponse::WhatsAppSendMessageResponse(
            WhatsAppSendMessageResponse {
                message_status,
                connection_status,
            },
        )),
    }
}

pub fn get_router() -> router::Router<AppResponse, AppError> {
    let mut router = router::Router::for_child("whatsapp".to_string());

    router.add_route(router::Route {
        path: "info".to_string(),
        handler: get_whatsapp_info,
    });

    router.add_route(router::Route {
        path: "start_connection".to_string(),
        handler: start_connection,
    });

    router.add_route(router::Route {
        path: "send_message".to_string(),
        handler: send_message,
    });

    router
}
