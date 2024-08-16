mod authentication_handler;
mod connection_state_handler;
pub mod functions;
mod requests;
mod tdlib;
mod classes;

pub use authentication_handler::{AuthorizationHandler, ConsoleAuthorizationHandler};
pub use connection_state_handler::{ConnectionHandler, ConsoleConnectionHandler};
pub use requests::TdLibType;
pub use classes::*;

use log::{debug, error, trace};
use requests::{AuthorizationState, TDLibResponse, TelegramRequest};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tdlib::{new_client, receive, send};
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;
use tokio::time;

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
    pub fn for_testing() -> Arc<Self> {
        Arc::new(TelegramClient {
            request_handles: Arc::new(Mutex::new(HashMap::new())),
            listener_task: None,
            client_id: 1,
            version: None,
            commit_hash: None,
            authorization_state: None,
        })
    }

    pub async fn init<H, C>() -> Arc<Self>
    where
        H: AuthorizationHandler + 'static,
        C: ConnectionHandler + 'static,
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

        let (mut auth_rx, mut conn_rx) = client.start_receiving().await;

        let client = Arc::new(client);

        let _ = client
            .send(functions::SetLogVerbosityLevel::new(&client, 0))
            .await;

        let _ = client
            .send(functions::GetAuthorizationState::new(&client))
            .await;

        let authorization_handler = H::new(client.clone());
        let connection_state_handler = C::new();

        tokio::spawn(async move {
            loop {
                let response = auth_rx.recv().await;

                if response.is_none() {
                    continue;
                }

                let response = response.unwrap();
                let authorization_state = response.authorization_state.unwrap();

                match authorization_state.state {
                    requests::AuthorizationState::AuthorizationStateWaitTdlibParameters => {
                        authorization_handler.handle_set_tdlib_params().await;
                    }
                    requests::AuthorizationState::AuthorizationStateWaitPhoneNumber => {
                        authorization_handler.handle_wait_phone_number().await;
                    }
                    requests::AuthorizationState::AuthorizationStateWaitCode => {
                        authorization_handler.handle_wait_code().await;
                    }
                    AuthorizationState::AuthorizationStateWaitPassword => {
                        authorization_handler
                            .handle_wait_password(authorization_state.password_hint)
                            .await;
                    }
                    requests::AuthorizationState::AuthorizationStateWaitRegistration => {
                        authorization_handler.handle_wait_code().await;
                    }
                    AuthorizationState::AuthorizationStateWaitOtherDeviceConfirmation => {
                        authorization_handler
                            .handle_wait_other_device_confirmation(
                                authorization_state.link.unwrap(),
                            )
                            .await;
                    }
                    requests::AuthorizationState::AuthorizationStateWaitEmailAddress => {
                        authorization_handler.handle_wait_email_address().await;
                    }
                    requests::AuthorizationState::AuthorizationStateWaitEmailCode => {
                        authorization_handler.handle_wait_email_code().await;
                    }
                    requests::AuthorizationState::AuthorizationStateReady => {
                        authorization_handler.handle_ready().await;
                    }
                    requests::AuthorizationState::AuthorizationStateLoggingOut => {
                        authorization_handler.handle_logging_out().await;
                    }
                    requests::AuthorizationState::AuthorizationStateClosing => {
                        authorization_handler.handle_closing().await;
                    }
                    requests::AuthorizationState::AuthorizationStateClosed => {
                        authorization_handler.handle_closed().await;
                    }
                }
            }
        });

        tokio::spawn(async move {
            loop {
                let response = conn_rx.recv().await;

                if response.is_none() {
                    continue;
                }

                let response = response.unwrap();
                let connection_state = response.state.unwrap();

                match connection_state.state {
                    requests::ConnectionState::ConnectionStateWaitingForNetwork => {
                        connection_state_handler.handle_waiting_for_network().await;
                    }
                    requests::ConnectionState::ConnectionStateConnectingToProxy => {
                        connection_state_handler.handle_connecting_to_proxy().await;
                    }
                    requests::ConnectionState::ConnectionStateConnecting => {
                        connection_state_handler.handle_connecting().await;
                    }
                    requests::ConnectionState::ConnectionStateUpdating => {
                        connection_state_handler.handle_updating().await;
                    }
                    requests::ConnectionState::ConnectionStateReady => {
                        connection_state_handler.handle_ready().await;
                    }
                }
            }
        });

        client
    }

    /// Start listening for tdlib events and returns a receiver for authentication messages
    /// # Returns
    ///
    /// A tuple of Receivers
    ///
    /// 1. Listener for authentication messages
    /// 2. Listener for connection state updates
    async fn start_receiving(
        &mut self,
    ) -> (
        tokio::sync::mpsc::Receiver<TDLibResponse>,
        tokio::sync::mpsc::Receiver<TDLibResponse>,
    ) {
        let (auth_tx, auth_rx) = tokio::sync::mpsc::channel::<TDLibResponse>(100);
        let (conn_tx, conn_rx) = tokio::sync::mpsc::channel::<TDLibResponse>(100);

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
                    debug!("Failed to parse response from tdlib: {}", e);
                    continue;
                }

                let response = response.unwrap();

                if let None = response.extra.as_ref() {
                    match response.td_type {
                        TdLibType::UpdateAuthorizationState => {
                            debug!(target: LOG_TARGET, "Found authorization state update event: {}", event);
                            auth_tx.send(response).await.unwrap_or_default();
                        }
                        TdLibType::UpdateConnectionState => {
                            debug!(target: LOG_TARGET, "Found connection state update event: {}", event);
                            conn_tx.send(response).await.unwrap_or_default();
                        }
                        _ => {
                            trace!(target: LOG_TARGET, "No handle found in response {}", event);
                            continue;
                        }
                    }
                } else if let Some(handle) = response.extra.as_ref() {
                    if let Some(sender) = request_handles_arc.lock().await.remove(handle) {
                        debug!(target: LOG_TARGET, "Found active handle for response: {}", event);
                        let _ = sender.send(response);
                    } else {
                        debug!(target: LOG_TARGET, "No active handle for response: {}", event);
                    }
                }
            }
        });

        self.listener_task = Some(join_handle);

        (auth_rx, conn_rx)
    }

    /// Generate a new random handle for @extra field in tdlib
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

    /// Send a request to tdlib
    pub async fn send(&self, request: impl TelegramRequest) -> Result<TDLibResponse, String> {
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

                match time::timeout(Duration::from_secs(5), rx).await {
                    Ok(response) => response.map_err(|err| (err.to_string())),
                    Err(e) => {
                        error!("Timed out waiting for response from tdlib: {}", e);
                        self.request_handles.lock().await.remove(&request.extra());
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => {
                error!("Failed to serialize request to tdlib: {}", e);
                Err(e.to_string())
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
