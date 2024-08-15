use std::sync::Arc;

use crate::app::{validate_payload, AppState};
use crate::validation::validate_phone_number;
use crate::whatsapp::WhatsAppStatus;
use axum::extract::State;
use axum::{debug_handler, http::StatusCode, Json};
use log::info;
use mockall_double::double;
use serde;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[double]
use crate::whatsapp::WhatsAppBot;

static LOG_TARGET: &str = "Resend signin code";

#[derive(Serialize, Deserialize, Validate)]
pub struct ResendSigninCodePayload {
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_number: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResendSigninCodeStatus {
    UserNotFound,
    CodeExpired,
    TargetNotOnWhatsApp,
    MessageSent,
    WhatsAppError,
}

#[derive(Serialize)]
pub struct ResendSigninCodeResponse {
    status: ResendSigninCodeStatus,
}

#[debug_handler]
pub async fn resend_signin_code(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResendSigninCodePayload>,
) -> (StatusCode, Json<ResendSigninCodeResponse>) {
    validate_payload(&payload);

    let user = state.db.user.find_user(&payload.phone_number).await;

    if let None = user {
        info!(target: LOG_TARGET, "User not found with phone number: {}", payload.phone_number);
        return (
            StatusCode::NOT_FOUND,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::UserNotFound,
            }),
        );
    }

    info!(target: LOG_TARGET, "User found with phone number: {}", payload.phone_number);

    let signin_code = state
        .db
        .signin_code
        .find_signin_code(&payload.phone_number)
        .await;

    if let None = signin_code {
        info!(target: LOG_TARGET, "Signin code not found for {}", payload.phone_number);
        return (
            StatusCode::NOT_FOUND,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::CodeExpired,
            }),
        );
    }

    let signin_code = signin_code.unwrap();

    if signin_code.created_at.to_utc().time() - chrono::Utc::now().time() > chrono::Duration::minutes(10) {
        info!(target: LOG_TARGET, "Signin code expired for {}", payload.phone_number);
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::CodeExpired,
            }),
        );
    }

    info!(target: LOG_TARGET, "Sending new signin code to {}", &payload.phone_number);
    let response = WhatsAppBot::send_message(
        payload.phone_number.clone(),
        format!("Your signin code is: {}", &signin_code.code),
    )
    .await;

    match response.status {
        WhatsAppStatus::TargetNotOnWhatsApp => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TargetNotOnWhatsApp,
                }),
            );
        }
        WhatsAppStatus::MessageSent => {}
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::WhatsAppError,
                }),
            );
        }
    };

    info!(target: LOG_TARGET, "Signin code created for {}", payload.phone_number);
    (
        StatusCode::OK,
        Json(ResendSigninCodeResponse {
            status: ResendSigninCodeStatus::MessageSent,
        }),
    )
}
