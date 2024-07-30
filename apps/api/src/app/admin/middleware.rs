use core::str;

use crate::{app::AppErrorResponse, APP_SETTINGS};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use base64::{self, Engine};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn jwt_middleware(req: Request<Body>, next: Next) -> Result<Response, impl IntoResponse> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            let base64_encoded = &APP_SETTINGS.get().unwrap().admin_public_key;
            let public_key = base64::prelude::BASE64_STANDARD
                .decode(base64_encoded)
                .unwrap();

            let decoding_key = DecodingKey::from_rsa_pem(public_key.as_slice()).map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(AppErrorResponse {
                        error_message: "Invalid public key token".to_string(),
                    }),
                )
            });

            if let Err(e) = decoding_key {
                return Err(e);
            }

            let validation = Validation::new(Algorithm::RS256);

            let result =
                decode::<Claims>(token, &decoding_key.unwrap(), &validation).map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(AppErrorResponse {
                            error_message: "Unauthorized authentication token".to_string(),
                        }),
                    )
                });

            if let Err(e) = result {
                return Err(e);
            }

            return Ok(next.run(req).await);
        }
    }

    Err((
        StatusCode::UNAUTHORIZED,
        Json(AppErrorResponse {
            error_message: "Missing authentication token".to_string(),
        }),
    ))
}
