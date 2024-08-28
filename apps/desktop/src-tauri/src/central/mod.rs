use crate::sync::SyncEvent;
use base64::Engine;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use log::debug;
use rsa::pkcs1v15::SigningKey;
use rsa::signature::SignatureEncoding;
use rsa::{pkcs1::DecodeRsaPrivateKey, sha2::Sha256, signature::SignerMut, RsaPrivateKey};
use serde::{Deserialize, Serialize};
use tauri::api::http::{Body, ClientBuilder, HttpRequestBuilder, ResponseData};
use tauri::http::header::HeaderMap;

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
struct UploadChunkPayload {
    chunk: Vec<SyncEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CheckSyncAvailabilityError {
    CenterNotFound,
    CenterSignatureInvalid,
    Base64DecodeError,
    PrivateKeyParseError,
    SignatureGenerationError,
    NetworkError,
    ResponseReadError,
    UnknownError,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SyncUploadChunkError {
    SerializationError,
    CenterNotFound,
    SignatureInvalid,
    Base64DecodeError,
    PrivateKeyParseError,
    SignatureGenerationError,
    NetworkError,
    ResponseReadError,
    UnknownError,
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

            // Convert the RSA key into the correct format for jsonwebtoken
            let encoding_key = EncodingKey::from_rsa_der(&private_key_der);

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

    async fn sign_chunk(
        body: &String,
        private_key: &String,
    ) -> Result<String, SyncUploadChunkError> {
        let body = body.clone();
        let private_key = private_key.clone();

        tokio::task::spawn_blocking(move || {
            debug!(target: LOG_TARGET, "Loading private key");

            let private_key_der = base64::prelude::BASE64_STANDARD
                .decode(private_key)
                .map_err(|_| SyncUploadChunkError::Base64DecodeError)?;

            let private_key = RsaPrivateKey::from_pkcs1_der(&private_key_der)
                .map_err(|_| SyncUploadChunkError::PrivateKeyParseError)?;

            let mut signing_key = SigningKey::<Sha256>::new(private_key);

            let signature = signing_key.sign(body.as_bytes());

            Ok(base64::prelude::BASE64_STANDARD.encode(signature.to_bytes()))
        })
        .await
        .map_err(|_| SyncUploadChunkError::SignatureGenerationError)?
    }

    pub async fn sync_upload_chunk(
        events: &[SyncEvent],
        private_key: &String,
        center_id: &String,
    ) -> Result<(), SyncUploadChunkError> {
        let url = format!("{}/sync/upload_chunk", CENTRAL_API);
        let client = ClientBuilder::new().build().unwrap();
        let request = HttpRequestBuilder::new("POST", url).unwrap();

        let payload = UploadChunkPayload {
            chunk: events.to_vec(),
        };

        let chunk = match serde_json::to_string(&payload) {
            Ok(json) => Ok(json),
            Err(_) => Err(SyncUploadChunkError::SerializationError),
        }?;

        let signature = Self::sign_chunk(&chunk, &private_key).await?;

        let mut headers = HeaderMap::new();

        headers.append("Content-Type", "application/json".parse().unwrap());
        headers.append("Signature", signature.parse().unwrap());
        headers.append("Center-ID", center_id.parse().unwrap());

        let request = request
            .body(Body::Bytes(chunk.bytes().collect()))
            .headers(headers);

        debug!(target: LOG_TARGET, "Sending request to Central API");
        let response = client
            .send(request)
            .await
            .map_err(|_| SyncUploadChunkError::NetworkError)?;

        debug!(target: LOG_TARGET, "Parsing response");
        let ResponseData { data, .. } = response
            .read()
            .await
            .map_err(|_| SyncUploadChunkError::ResponseReadError)?;

        let status = data
            .get("status")
            .ok_or(SyncUploadChunkError::ResponseReadError)?;

        match status {
            serde_json::Value::String(status) => match status.as_str() {
                "center_not_found" => Err(SyncUploadChunkError::CenterNotFound),
                "signature_invalid" => Err(SyncUploadChunkError::SignatureInvalid),
                "accepted" => Ok(()),
                _ => Err(SyncUploadChunkError::ResponseReadError),
            },
            _ => Err(SyncUploadChunkError::ResponseReadError),
        }
    }

    pub async fn check_sync_availability(
        center_id: &String,
        private_key: &String,
    ) -> Result<(), CheckSyncAvailabilityError> {
        debug!(target: LOG_TARGET, "Checking sync availability");
        let url = format!("{}/sync/check_sync_availability", CENTRAL_API);

        let client = ClientBuilder::new().build().unwrap();

        let request = HttpRequestBuilder::new("POST", url).unwrap();

        debug!(target: LOG_TARGET, "Generating signature");
        let signature = Self::generate_signature(center_id.clone(), private_key.clone()).await?;

        let request = request.body(Body::Json(
            serde_json::to_value(CheckSyncAvailabilityPayload {
                center_id: center_id.clone(),
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
