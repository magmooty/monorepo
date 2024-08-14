use log::info;
use qr2term;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::{
    functions::{CheckAuthenticationPassword, RequestQrCodeAuthentication, SetTdLibParameters},
    TelegramClient,
};

static LOG_TARGET: &str = "Console Authentication Handler";

pub trait AuthorizationHandler: Send + Sync {
    async fn handle_set_tdlib_params(&self) -> ();
    async fn handle_set_phone_number(&self) -> ();
    async fn handle_wait_other_device_confirmation(&self, link: String) -> ();
    async fn handle_wait_password(&self, hint: Option<String>) -> ();
    async fn handle_status_ready(&self) -> ();
}

pub struct ConsoleAuthorizationHandler {
    client: Arc<TelegramClient>,
}

impl ConsoleAuthorizationHandler {
    pub fn new(client: Arc<TelegramClient>) -> Self {
        Self { client }
    }
}

impl AuthorizationHandler for ConsoleAuthorizationHandler {
    async fn handle_set_tdlib_params(&self) -> () {
        let client = self.client.clone();

        client
            .send(SetTdLibParameters::new(
                &client,
                24977003,
                "6adc83372bceff3460093e1846796d49".to_string(),
            ))
            .await
            .unwrap();
    }

    async fn handle_set_phone_number(&self) -> () {
        let client = self.client.clone();

        client
            .send(RequestQrCodeAuthentication::new(&client))
            .await
            .unwrap();
    }

    async fn handle_wait_other_device_confirmation(&self, link: String) {
        qr2term::print_qr(link).unwrap();
    }

    async fn handle_wait_password(&self, _: Option<String>) -> () {
        info!(target: LOG_TARGET, "QR code scanned, waiting for password");

        let mut reader = BufReader::new(io::stdin());
        let mut password = String::new();

        print!("Telegram password: ");

        reader.read_line(&mut password).await.unwrap();

        let client = self.client.clone();

        client
            .send(CheckAuthenticationPassword::new(&client, password))
            .await
            .unwrap();
    }

    async fn handle_status_ready(&self) {
        println!("Telegram connected and logged in");
    }
}
