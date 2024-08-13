use axum::debug_handler;
use axum::response::IntoResponse;
use http::header;
use log::info;
use mockall_double::double;
use qrcode_generator::{self, QrCodeEcc};

#[double]
use crate::whatsapp::WhatsAppBot;

static LOG_TARGET: &str = "Generate QR code";

#[debug_handler]
pub async fn generate_whatsapp_qr_code() -> impl IntoResponse {
    info!(target: LOG_TARGET, "Generating new WhatsApp QR code");

    let response = WhatsAppBot::start_connection().await;

    let qr_input = response.code.clone();
    let qr_vec = tokio::task::spawn_blocking(move || {
        qrcode_generator::to_png_to_vec(qr_input, QrCodeEcc::High, 1024).unwrap()
    })
    .await
    .unwrap()
    .clone();

    let headers = [(header::CONTENT_TYPE, "image/png".to_string())];

    (headers, qr_vec)
}
