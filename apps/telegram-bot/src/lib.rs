mod authentication_handler;
mod functions;
mod requests;
mod tdlib;

use authentication_handler::{AuthorizationHandler, ConsoleAuthorizationHandler};
use log::{debug, error};
use requests::{AuthorizationState, TDLibResponse, TdLibType, TelegramRequest};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tdlib::{new_client, receive, send};
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;

#[macro_use]
extern crate telegram_macros;

static LOG_TARGET: &str = "Telegram";

pub struct TelegramClient {
    request_handles: Arc<Mutex<HashMap<String, oneshot::Sender<TDLibResponse>>>>,
    listener_task: Option<JoinHandle<()>>,
    pub client_id: i32,
    pub version: Option<String>,
    pub commit_hash: Option<String>,
    pub authorization_state: Option<AuthorizationState>,
}

impl TelegramClient {
    async fn init<H>() -> Arc<Self>
    where
        H: AuthorizationHandler + 'static,
    {
        let client_id = new_client();
        let mut client = TelegramClient {
            client_id,
            request_handles: Arc::new(Mutex::new(HashMap::new())),
            listener_task: None,
            version: None,
            commit_hash: None,
            authorization_state: None,
        };

        let mut rx = client.start_receiving().await;

        let client = Arc::new(client);

        let _ = client
            .send(functions::SetLogVerbosityLevel::new(&client, 0))
            .await;

        let _ = client
            .send(functions::GetAuthorizationState::new(&client))
            .await;

        let authorization_handler = H::new(client.clone());

        tokio::spawn(async move {
            loop {
                let response = rx.recv().await;

                if response.is_none() {
                    continue;
                }

                let response = response.unwrap();
                let authorization_state = response.authorization_state.unwrap();

                match authorization_state.state {
                    AuthorizationState::AuthorizationStateWaitTdlibParameters => {
                        authorization_handler.handle_set_tdlib_params().await;
                    }
                    AuthorizationState::AuthorizationStateWaitPhoneNumber => {
                        authorization_handler.handle_set_phone_number().await;
                    }
                    AuthorizationState::AuthorizationStateWaitOtherDeviceConfirmation => {
                        authorization_handler
                            .handle_wait_other_device_confirmation(
                                authorization_state.link.unwrap(),
                            )
                            .await;
                    }
                    AuthorizationState::AuthorizationStateWaitPassword => {
                        authorization_handler
                            .handle_wait_password(authorization_state.password_hint)
                            .await;
                    }
                    AuthorizationState::AuthorizationStateReady => {
                        authorization_handler.handle_status_ready().await;
                    }
                    AuthorizationState::AuthorizationStateWaitCode => {}
                }
            }
        });

        client
    }

    async fn start_receiving(&mut self) -> tokio::sync::mpsc::Receiver<TDLibResponse> {
        let (tx, rx) = tokio::sync::mpsc::channel::<TDLibResponse>(100);

        let request_handles_arc = self.request_handles.clone();

        let join_handle = tokio::spawn(async move {
            loop {
                let response = tokio::task::spawn_blocking(move || receive(300.0)).await;

                if let Err(e) = response {
                    error!("Failed to receive response from tdlib: {}", e.to_string());
                    continue;
                }

                let response = response.unwrap();

                if let None = response {
                    debug!(target: LOG_TARGET, "No received response from tdlib");
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
                    match response.td_type {
                        TdLibType::UpdateAuthorizationState => {
                            debug!(target: LOG_TARGET, "Found authorization state update event: {}", event);
                            tx.send(response).await.unwrap_or_default();
                        }
                        _ => {
                            debug!(target: LOG_TARGET, "No handle found in response: {}", event);
                            continue;
                        }
                    }
                } else if let Some(handle) = response.extra.as_ref() {
                    if let Some(sender) = request_handles_arc.lock().await.remove(handle) {
                        let _ = sender.send(response);
                    } else {
                        debug!(target: LOG_TARGET, "No active handle for response: {}", event);
                    }
                }
            }
        });

        self.listener_task = Some(join_handle);

        rx
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

        debug!(target: LOG_TARGET,
            "Assigning a request handle for request: {}",
            request.extra()
        );

        self.request_handles
            .lock()
            .await
            .insert(request.extra(), tx);

        match serde_json::to_string(&request) {
            Ok(request_json) => {
                debug!(target: LOG_TARGET, "Sending request to tdlib: {}", request.extra());

                send(self.client_id, request_json.as_str());

                debug!(target: LOG_TARGET, "Sent request to tdlib: {}", request.extra());

                rx.await.map_err(|_| (()))
            }
            Err(e) => {
                error!("Failed to serialize request to tdlib: {}", e);
                Err(())
            }
        }
    }
}

impl Drop for TelegramClient {
    fn drop(&mut self) {
        if let Some(listener_task) = &self.listener_task {
            listener_task.abort();
        }
    }
}

pub async fn initialize_telegram() {
    let _ = TelegramClient::init::<ConsoleAuthorizationHandler>().await;
}
