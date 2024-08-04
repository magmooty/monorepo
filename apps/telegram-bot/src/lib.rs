mod functions;
mod requests;
mod tdlib;

use log::{debug, error};
use requests::{TDLibResponse, TelegramRequest};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tdlib::{new_client, receive, send};
use tokio::sync::oneshot;

pub struct TelegramClient {
    request_handles: Arc<Mutex<HashMap<String, oneshot::Sender<TDLibResponse>>>>,
    pub client_id: i32,
    pub version: Option<String>,
    pub commit_hash: Option<String>,
}

impl TelegramClient {
    fn new() -> Self {
        let client_id = new_client();
        Self {
            client_id,
            request_handles: Arc::new(Mutex::new(HashMap::new())),
            version: None,
            commit_hash: None,
        }
    }

    fn start_receiving(&self) {
        let request_handles_arc = self.request_handles.clone();

        tokio::spawn(async move {
            loop {
                let response = tokio::task::spawn_blocking(move || receive(10.0)).await;

                if let Err(e) = response {
                    error!("Failed to receive response from tdlib: {}", e.to_string());
                    continue;
                }

                let response = response.unwrap();

                if let None = response {
                    debug!("No received response from tdlib");
                    continue;
                }

                let event = response.unwrap();

                let response = serde_json::from_str::<TDLibResponse>(event.as_str());

                if let Err(e) = response {
                    error!("Failed to parse response from tdlib: {}", e);
                    continue;
                }

                let response = response.unwrap();

                if let None = response.extra.as_ref() {
                    debug!("No handle found in response: {}", event);
                    continue;
                }

                let handle = response.extra.as_ref().unwrap();

                if let Some(sender) = request_handles_arc.lock().unwrap().remove(handle) {
                    let _ = sender.send(response);
                } else {
                    debug!("No active handle for response: {}", event);
                }
            }
        });
    }

    fn generate_extra_handle(&self) -> String {
        format!(
            "{}-{}",
            self.client_id,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Cannot get system time")
                .as_millis()
        )
    }

    async fn send(&self, request: impl TelegramRequest) -> Result<TDLibResponse, ()> {
        let (tx, rx) = oneshot::channel();

        debug!(
            "Assigning a request handle for request: {}",
            request.extra()
        );

        self.request_handles
            .lock()
            .unwrap()
            .insert(request.extra(), tx);

        match serde_json::to_string(&request) {
            Ok(request_json) => {
                debug!("Sening request to tdlib: {}", request.extra());

                send(self.client_id, request_json.as_str());

                debug!("Sent request to tdlib: {}", request.extra());

                rx.await.map_err(|_| (()))
            }
            Err(e) => {
                error!("Failed to serialize request to tdlib: {}", e);
                Err(())
            }
        }
    }
}

pub async fn initialize_telegram() {
    let client = TelegramClient::new();

    client.start_receiving();

    let _ = client
        .send(functions::SetLogVerbosityLevel::new(&client, 1))
        .await;

    // send(
    //     client,
    //     r#"{"@type":"getAuthorizationState", "@extra":"getAuth"}"#,
    // );
}
