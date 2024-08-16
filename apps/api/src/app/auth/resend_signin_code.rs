use std::sync::Arc;

use crate::app::common::MessagingChannel;
use crate::app::{validate_payload, AppState};
use crate::validation::validate_phone_number;
use crate::whatsapp::WhatsAppStatus;
use axum::extract::State;
use axum::{debug_handler, http::StatusCode, Json};
use log::info;
use mockall_double::double;
use serde;
use serde::{Deserialize, Serialize};
use telegram_bot::functions::{CreatePrivateChat, SearchUserByPhoneNumber, SendMessage};
use telegram_bot::{TdLibType, TelegramChat, TelegramUser};
use utoipa::ToSchema;
use validator::Validate;

#[double]
use crate::whatsapp::WhatsAppBot;

static LOG_TARGET: &str = "Resend signin code";

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct ResendSigninCodePayload {
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_number: String,

    pub channel: MessagingChannel,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResendSigninCodeStatus {
    UserNotFound,
    CodeExpired,
    TargetNotOnWhatsApp,
    TargetNotOnTelegram,
    MessageSent,
    WhatsAppError,
    TelegramError,
}

#[derive(Serialize, ToSchema)]
pub struct ResendSigninCodeResponse {
    status: ResendSigninCodeStatus,
}

async fn send_signin_code_whatsapp(
    payload: &ResendSigninCodePayload,
    code: String,
) -> Result<(), (StatusCode, Json<ResendSigninCodeResponse>)> {
    info!(target: LOG_TARGET, "Sending new signin code to {}", &payload.phone_number);
    let response = WhatsAppBot::send_message(
        payload.phone_number.clone(),
        format!("Your signin code is: {}", code),
    )
    .await;

    match response.status {
        WhatsAppStatus::TargetNotOnWhatsApp => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TargetNotOnWhatsApp,
                }),
            ));
        }
        WhatsAppStatus::MessageSent => Ok(()),
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::WhatsAppError,
                }),
            ));
        }
    }
}

async fn send_signin_code_telegram(
    state: &Arc<AppState>,
    payload: &ResendSigninCodePayload,
    code: String,
) -> Result<(), (StatusCode, Json<ResendSigninCodeResponse>)> {
    info!(target: LOG_TARGET, "Sending new signin code to {}", &payload.phone_number);

    let tg_user = state
        .telegram
        .send(SearchUserByPhoneNumber::new(
            &state.telegram,
            payload.phone_number.clone(),
        ))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    if matches!(tg_user.td_type, TdLibType::Error)
        && tg_user.data.get("code").is_some()
        && tg_user.data.get("code").unwrap().as_i64().unwrap() == 404
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::TargetNotOnTelegram,
            }),
        ));
    }

    let tg_user = serde_json::from_value::<TelegramUser>(tg_user.data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::TelegramError,
            }),
        )
    })?;

    let chat = state
        .telegram
        .send(CreatePrivateChat::new(&state.telegram, tg_user.id))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    let chat = serde_json::from_value::<TelegramChat>(chat.data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::TelegramError,
            }),
        )
    })?;

    let message = state
        .telegram
        .send(SendMessage::new(
            &state.telegram,
            chat.id,
            format!("Your signin code is: {}", code),
        ))
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    match message.td_type {
        TdLibType::Error => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResendSigninCodeResponse {
                    status: ResendSigninCodeStatus::TelegramError,
                }),
            ));
        }
        _ => {}
    }

    Ok(())
}

#[debug_handler]
#[utoipa::path(
    post,
    tag = "Authorization",
    path = "/auth/resend_signin_code",
    request_body = ResendSigninCodePayload,
    responses(
        (status = OK, description = "Sent sign in code", body = ResendSigninCodeResponse, example = json!({ "status": "message_sent" })),
        (status = NOT_FOUND, description = "User was not found", body = ResendSigninCodeResponse, example = json!({ "status": "user_not_found" })),
        (status = BAD_REQUEST, description = "Target is not on WhatsApp", body = ResendSigninCodeResponse, example = json!({ "status": "target_not_on_whatsapp" })),
        (status = UNAUTHORIZED, description = "Signin code has expired", body = ResendSigninCodeResponse, example = json!({ "status": "code_expired" })),
        (status = INTERNAL_SERVER_ERROR, description = "WhatsApp error", body = ResendSigninCodeResponse, example = json!({ "status": "whatsapp_error" }))
    )
)]
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
            StatusCode::UNAUTHORIZED,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::CodeExpired,
            }),
        );
    }

    let signin_code = signin_code.unwrap();

    if signin_code.created_at.to_utc().time() - chrono::Utc::now().time()
        > chrono::Duration::minutes(10)
    {
        info!(target: LOG_TARGET, "Signin code expired for {}", payload.phone_number);
        return (
            StatusCode::UNAUTHORIZED,
            Json(ResendSigninCodeResponse {
                status: ResendSigninCodeStatus::CodeExpired,
            }),
        );
    }

    info!(target: LOG_TARGET, "Resending new signin code to {}", &payload.phone_number);
    let code = format!("Your signin code is: {}", &signin_code.code);

    match payload.channel {
        MessagingChannel::WhatsApp => {
            if let Err(error_response) = send_signin_code_whatsapp(&payload, code).await {
                return error_response;
            }
        }
        MessagingChannel::Telegram => {
            if let Err(error_response) = send_signin_code_telegram(&state, &payload, code).await {
                return error_response;
            }
        }
    }

    info!(target: LOG_TARGET, "Signin code created for {}", payload.phone_number);
    (
        StatusCode::OK,
        Json(ResendSigninCodeResponse {
            status: ResendSigninCodeStatus::MessageSent,
        }),
    )
}
