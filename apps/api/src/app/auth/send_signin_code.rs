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

static LOG_TARGET: &str = "Send signin code";

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SendSigninCodePayload {
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_number: String,

    pub channel: MessagingChannel,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SendSigninCodeStatus {
    TargetNotOnWhatsApp,
    MessageSent,
    WhatsAppError,
    TargetNotOnTelegram,
    TelegramError,
}

#[derive(Serialize, ToSchema)]
pub struct SendSigninCodeResponse {
    status: SendSigninCodeStatus,
}

async fn send_signin_code_whatsapp(
    payload: &SendSigninCodePayload,
    code: String,
) -> Result<(), (StatusCode, Json<SendSigninCodeResponse>)> {
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
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::TargetNotOnWhatsApp,
                }),
            ));
        }
        WhatsAppStatus::MessageSent => Ok(()),
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::WhatsAppError,
                }),
            ));
        }
    }
}

async fn send_signin_code_telegram(
    state: &Arc<AppState>,
    payload: &SendSigninCodePayload,
    code: String,
) -> Result<(), (StatusCode, Json<SendSigninCodeResponse>)> {
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
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    if matches!(tg_user.td_type, TdLibType::Error)
        && tg_user.data.get("code").is_some()
        && tg_user.data.get("code").unwrap().as_i64().unwrap() == 404
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(SendSigninCodeResponse {
                status: SendSigninCodeStatus::TargetNotOnTelegram,
            }),
        ));
    }

    let tg_user = serde_json::from_value::<TelegramUser>(tg_user.data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SendSigninCodeResponse {
                status: SendSigninCodeStatus::TelegramError,
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
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    let chat = serde_json::from_value::<TelegramChat>(chat.data).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(SendSigninCodeResponse {
                status: SendSigninCodeStatus::TelegramError,
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
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::TelegramError,
                }),
            )
        })?;

    match message.td_type {
        TdLibType::Error => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SendSigninCodeResponse {
                    status: SendSigninCodeStatus::TelegramError,
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
    path = "/auth/send_signin_code",
    request_body = SendSigninCodePayload,
    responses(
        (status = CREATED, description = "Sent sign in code", body = SendSigninCodeResponse, example = json!({ "status": "message_sent" })),
        (status = BAD_REQUEST, description = "Target is not on Telegram", body = SendSigninCodeResponse, example = json!({ "status": "target_not_on_telegram" })),
        (status = BAD_REQUEST, description = "Target is not on WhatsApp", body = SendSigninCodeResponse, example = json!({ "status": "target_not_on_whatsapp" })),
        (status = INTERNAL_SERVER_ERROR, description = "WhatsApp error", body = SendSigninCodeResponse, example = json!({ "status": "whatsapp_error" })),
        (status = INTERNAL_SERVER_ERROR, description = "Telegram error", body = SendSigninCodeResponse, example = json!({ "status": "telegram_error" }))
    )
)]
pub async fn send_signin_code(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SendSigninCodePayload>,
) -> (StatusCode, Json<SendSigninCodeResponse>) {
    validate_payload(&payload);

    let user = state.db.user.find_user(&payload.phone_number).await;

    if let None = user {
        info!(
            target: LOG_TARGET,
            "Creating a new user with phone number: {}",
            payload.phone_number
        );
        state.db.user.create_user(&payload.phone_number).await;
    } else {
        info!(target: LOG_TARGET, "User found with phone number: {}", payload.phone_number);
    }

    info!(
        target: LOG_TARGET,
        "Deleting previous signin codes for {}",
        &payload.phone_number
    );
    state
        .db
        .signin_code
        .delete_previous_signin_codes(&payload.phone_number)
        .await;

    info!(target: LOG_TARGET, "Creating new signin code for {}", &payload.phone_number);
    let code = state
        .db
        .signin_code
        .create_signin_code(&payload.phone_number, 0)
        .await;

    info!(target: LOG_TARGET, "Sending new signin code to {}", &payload.phone_number);
    let code = format!("Your signin code is: {}", &code);

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
        StatusCode::CREATED,
        Json(SendSigninCodeResponse {
            status: SendSigninCodeStatus::MessageSent,
        }),
    )
}
