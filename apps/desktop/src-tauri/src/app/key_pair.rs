use base64::Engine;
use log::{debug, info};
use rand::rngs::OsRng;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};
use serde::Serialize;
use specta::Type;

static LOG_TARGET: &str = "Key pair";

#[derive(Serialize, Debug, Clone, Type)]
pub struct KeyPair {
    pub private_key: String,
    pub public_key: String,
}

#[tauri::command]
#[specta::specta]
pub async fn generate_key_pair() -> KeyPair {
    tokio::task::spawn_blocking(|| {
        info!(target: LOG_TARGET, "Generating new key pair");
        // Generate a key pair
        let mut rng = OsRng;
        let bits = 2048;

        debug!(target: LOG_TARGET, "Generating private key");
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

        debug!(target: LOG_TARGET, "Generating public key");
        let public_key = RsaPublicKey::from(&private_key);

        debug!(target: LOG_TARGET, "Converting private key to base64");
        let private_key = private_key
            .to_pkcs1_der()
            .expect("Failed to encode private key")
            .as_bytes()
            .to_vec();

        debug!(target: LOG_TARGET, "Converting public key to base64");
        let public_key = public_key
            .to_pkcs1_der()
            .expect("Failed to encode private key")
            .as_bytes()
            .to_vec();

        KeyPair {
            public_key: base64::prelude::BASE64_STANDARD.encode(public_key),
            private_key: base64::prelude::BASE64_STANDARD.encode(private_key),
        }
    })
    .await
    .expect("Failed to generate key pair")
}
