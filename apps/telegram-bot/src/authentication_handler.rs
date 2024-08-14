use log::info;
use qr2term;
use std::io::{self, BufRead, BufReader, Write};
use std::sync::Arc;

use crate::{
    functions::{CheckAuthenticationPassword, RequestQrCodeAuthentication, SetTdLibParameters},
    TelegramClient,
};

static LOG_TARGET: &str = "Telegram Console Authentication Handler";

#[async_trait::async_trait]
pub trait AuthorizationHandler: Send + Sync {
    fn new(client: Arc<TelegramClient>) -> Self;
    fn get_client(&self) -> Arc<TelegramClient>;

    async fn handle_set_tdlib_params(&self) -> ();
    async fn handle_wait_phone_number(&self) -> () {
        let client = self.get_client();

        client
            .send(RequestQrCodeAuthentication::new(&client))
            .await
            .unwrap();
    }
    async fn handle_wait_email_address(&self) -> () {
        let client = self.get_client();

        client
            .send(RequestQrCodeAuthentication::new(&client))
            .await
            .unwrap();
    }
    async fn handle_wait_email_code(&self) -> () {}
    async fn handle_wait_code(&self) -> () {}
    async fn handle_wait_other_device_confirmation(&self, link: String) -> ();
    async fn handle_wait_password(&self, hint: Option<String>) -> ();
    async fn handle_ready(&self) -> ();
    async fn handle_logging_out(&self) -> () {}
    async fn handle_closing(&self) -> () {}
    async fn handle_closed(&self) -> ();
}

pub struct ConsoleAuthorizationHandler {
    client: Arc<TelegramClient>,
}

#[async_trait::async_trait]
impl AuthorizationHandler for ConsoleAuthorizationHandler {
    fn new(client: Arc<TelegramClient>) -> Self {
        Self { client }
    }

    fn get_client(&self) -> Arc<TelegramClient> {
        self.client.clone()
    }

    async fn handle_set_tdlib_params(&self) -> () {
        let client = self.get_client();

        client
            .send(SetTdLibParameters::new(
                &client,
                24977003,
                "6adc83372bceff3460093e1846796d49".to_string(),
            ))
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

        io::stdout().flush().unwrap();

        reader.read_line(&mut password).unwrap();

        println!("Logging in...");

        io::stdout().flush().unwrap();

        let client = self.get_client();

        client
            .send(CheckAuthenticationPassword::new(
                &client,
                password.trim().to_string(),
            ))
            .await
            .unwrap();
    }

    async fn handle_ready(&self) {
        info!(target: LOG_TARGET, "Telegram connected and logged in");
    }

    async fn handle_closed(&self) {
        info!(target: LOG_TARGET, "Connection closed");
    }
}
