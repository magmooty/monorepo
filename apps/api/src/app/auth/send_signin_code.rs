use crate::app::validate_payload;
use crate::database::{Record, SigninCode, User};
use crate::validation::validate_phone_number;
use crate::whatsapp::WhatsAppConnectionStatus;
use crate::{whatsapp, DB};
use axum::{debug_handler, http::StatusCode, Json};
use log::info;
use rand::Rng;
use serde;
use serde::{Deserialize, Serialize};
use validator::Validate;

// the input to our `create_user` handler
#[derive(Deserialize, Validate)]
pub struct SendSigninCodePayload {
    #[validate(custom(function = "validate_phone_number"))]
    phone_number: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct SendSigninCodeResponse {
    status: String,
}

async fn delete_previous_signin_codes(phone_number: &String) -> () {
    DB.query("DELETE signin_code WHERE phone_number = $phone_number")
        .bind(("phone_number", phone_number))
        .await
        .unwrap();
}

async fn create_signin_code(phone_number: &String, iteration: i32) -> String {
    if iteration > 10 {
        panic!("Failed to create signin code after 10 attempts");
    }

    // Generate a random 8 digit code
    let code = tokio::spawn(async {
        let mut rng = rand::thread_rng();
        rng.gen_range(00000000..=99999999)
    })
    .await
    .unwrap();

    let signin_code_record = DB
        .create::<Vec<Record>>("signin_code")
        .content(SigninCode {
            phone_number: phone_number.clone(),
            code: code.to_string(),
        })
        .await
        .unwrap()
        .first()
        .take()
        .unwrap()
        .clone();

    DB.query("UPDATE user SET signin_code = $code")
        .bind(("code", signin_code_record.id))
        .await
        .unwrap();

    code.to_string()
}

async fn find_user(phone_number: &String) -> Option<User> {
    DB.query("SELECT * FROM user WHERE phone_number = $phone_number")
        .bind(("phone_number", phone_number))
        .await
        .unwrap()
        .take::<Option<User>>(0)
        .unwrap()
}

async fn create_user(phone_number: &String) -> Record {
    let user: Vec<Record> = DB
        .create("user")
        .content(User {
            first_name: None,
            last_name: None,
            phone_number: phone_number.clone(),
        })
        .await
        .unwrap();

    user.first().take().unwrap().clone()
}

#[debug_handler]
pub async fn send_signin_code(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<SendSigninCodePayload>,
) -> (StatusCode, Json<SendSigninCodeResponse>) {
    validate_payload(&payload);

    let user = find_user(&payload.phone_number).await;

    if let None = user {
        info!(
            "Creating a new user with phone number: {}",
            payload.phone_number
        );
        create_user(&payload.phone_number).await;
    } else {
        info!("User found with phone number: {}", payload.phone_number);
    }

    info!(
        "Deleting previous signin codes for {}",
        &payload.phone_number
    );
    delete_previous_signin_codes(&payload.phone_number).await;

    info!("Creating new signin code for {}", &payload.phone_number);
    let code = create_signin_code(&payload.phone_number, 0).await;

    info!("Sending new signin code to {}", &payload.phone_number);
    let response = whatsapp::send_message(
        payload.phone_number.clone(),
        format!("Your signin code is: {}", code),
    )
    .await
    .unwrap();

    match response.connection_status {
        WhatsAppConnectionStatus::TargetNotOnWhatsApp => {
            return (
                StatusCode::BAD_REQUEST,
                Json(SendSigninCodeResponse {
                    status: "target_not_on_whatsapp".to_string(),
                }),
            );
        }
        WhatsAppConnectionStatus::SignedIn => {}
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SendSigninCodeResponse {
                    status: "whatsapp_error".to_string(),
                }),
            );
        }
    };

    info!("Signin code created for {}", payload.phone_number);
    (
        StatusCode::CREATED,
        Json(SendSigninCodeResponse {
            status: "message_sent".to_string(),
        }),
    )
}
