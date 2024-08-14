use log::info;

static LOG_TARGET: &str = "Telegram Console Connection State Handler";

#[async_trait::async_trait]
pub trait ConnectionHandler: Send + Sync {
    fn new() -> Self;

    async fn handle_waiting_for_network(&self) -> ();
    async fn handle_connecting_to_proxy(&self) -> ();
    async fn handle_connecting(&self) -> ();
    async fn handle_updating(&self) -> ();
    async fn handle_ready(&self) -> ();
}

pub struct ConsoleConnectionHandler {}

#[async_trait::async_trait]
impl ConnectionHandler for ConsoleConnectionHandler {
    fn new() -> Self {
        Self {}
    }

    async fn handle_waiting_for_network(&self) -> () {
        info!(target: LOG_TARGET, "Waiting for network...");
    }

    async fn handle_connecting_to_proxy(&self) -> () {
        info!(target: LOG_TARGET, "Connecting to proxy...");
    }

    async fn handle_connecting(&self) -> () {
        info!(target: LOG_TARGET, "Connecting...");
    }

    async fn handle_updating(&self) -> () {
        info!(target: LOG_TARGET, "Updating...");
    }

    async fn handle_ready(&self) -> () {
        info!(target: LOG_TARGET, "Ready!");
    }
}
