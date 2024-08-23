use base64::Engine;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use log::debug;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use tauri::api::http::{Body, ClientBuilder, HttpRequestBuilder, ResponseData};

#[cfg(debug_assertions)]
static CENTRAL_API: &str = "http://127.0.0.1:4000";

#[cfg(not(debug_assertions))]
static CENTRAL_API: &str = "https://central.magmooty.com";

static LOG_TARGET: &str = "Central API";

#[derive(Serialize, Deserialize, Debug)]
struct CheckSyncAvailabilityPayload {
    pub center_id: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CheckSyncAvailabilityError {
    CenterNotFound,
    CenterSignatureInvalid,
    Base64DecodeError,
    PrivateKeyParseError,
    SignatureGenerationError,
    NetworkError,
    ResponseReadError,
}

pub struct CentralAPI {}

impl CentralAPI {
    async fn generate_signature(
        center_id: String,
        private_key: String,
    ) -> Result<String, CheckSyncAvailabilityError> {
        tokio::task::spawn_blocking(move || {
            debug!(target: LOG_TARGET, "Loading private key");

            let private_key_der = base64::prelude::BASE64_STANDARD
                .decode(private_key)
                .map_err(|_| CheckSyncAvailabilityError::Base64DecodeError)?;

            // Load RSA private key
            let rsa_private_key = Rsa::private_key_from_der(&private_key_der)
                .map_err(|_| CheckSyncAvailabilityError::PrivateKeyParseError)?
                .private_key_to_der()
                .map_err(|_| CheckSyncAvailabilityError::PrivateKeyParseError)?;

            // Convert the RSA key into the correct format for jsonwebtoken
            let encoding_key = EncodingKey::from_rsa_der(&rsa_private_key);

            // Define your claims
            let my_claims = serde_json::json!({ "center_id": center_id });

            // Create the header and set the algorithm to RS256
            let header = Header::new(Algorithm::RS256);

            debug!(target: LOG_TARGET, "Generating signed JWT");

            // Encode the token
            jsonwebtoken::encode(&header, &my_claims, &encoding_key)
                .map_err(|_| CheckSyncAvailabilityError::SignatureGenerationError)
        })
        .await
        .map_err(|_| CheckSyncAvailabilityError::SignatureGenerationError)?
    }

    pub async fn check_sync_availability(
        center_id: String,
        private_key: String,
    ) -> Result<(), CheckSyncAvailabilityError> {
        debug!(target: LOG_TARGET, "Checking sync availability");
        let url = format!("{}/sync/check_sync_availability", CENTRAL_API);

        let client = ClientBuilder::new().build().unwrap();

        let request = HttpRequestBuilder::new("POST", url).unwrap();

        debug!(target: LOG_TARGET, "Generating signature");
        let signature = Self::generate_signature(center_id.clone(), private_key.clone()).await?;

        println!("sig: {}", &signature);

        let request = request.body(Body::Json(
            serde_json::to_value(CheckSyncAvailabilityPayload {
                center_id,
                signature,
            })
            .unwrap(),
        ));

        debug!(target: LOG_TARGET, "Sending request to Central API");
        let response = client
            .send(request)
            .await
            .map_err(|_| CheckSyncAvailabilityError::NetworkError)?;

        debug!(target: LOG_TARGET, "Parsing response");
        let ResponseData { data, .. } = response
            .read()
            .await
            .map_err(|_| CheckSyncAvailabilityError::ResponseReadError)?;

        let status = data
            .get("status")
            .ok_or(CheckSyncAvailabilityError::ResponseReadError)?;

        match status {
            serde_json::Value::String(status) => match status.as_str() {
                "center_not_found" => Err(CheckSyncAvailabilityError::CenterNotFound),
                "center_signature_invalid" => {
                    Err(CheckSyncAvailabilityError::CenterSignatureInvalid)
                }
                "available" => Ok(()),
                _ => Err(CheckSyncAvailabilityError::ResponseReadError),
            },
            _ => Err(CheckSyncAvailabilityError::ResponseReadError),
        }
    }
}
