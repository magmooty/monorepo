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

static LOG_TARGET: &str = "Send signin code";

// the input to our `create_user` handler
#[derive(Serialize, Deserialize, Validate)]
pub struct SendSigninCodePayload {
    #[validate(custom(function = "validate_phone_number"))]
    pub phone_number: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct SendSigninCodeResponse {
    status: String,
}

#[debug_handler]
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
    let response = WhatsAppBot::send_message(
        payload.phone_number.clone(),
        format!("Your signin code is: {}", code),
    )
    .await;

    match response.status {
        WhatsAppStatus::TargetNotOnWhatsApp => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SendSigninCodeResponse {
                    status: "target_not_on_whatsapp".to_string(),
                }),
            );
        }
        WhatsAppStatus::MessageSent => {}
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SendSigninCodeResponse {
                    status: "whatsapp_error".to_string(),
                }),
            );
        }
    };

    info!(target: LOG_TARGET, "Signin code created for {}", payload.phone_number);
    (
        StatusCode::CREATED,
        Json(SendSigninCodeResponse {
            status: "message_sent".to_string(),
        }),
    )
}
