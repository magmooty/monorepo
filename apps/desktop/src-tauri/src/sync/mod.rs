use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::Surreal;
use tokio::time::{sleep, Duration};

use crate::app::{get_global_key, GlobalKey};
use crate::central::CentralAPI;

static LOG_TARGET: &str = "Sync";

#[derive(Serialize, Deserialize, Debug)]
pub struct SyncEvent {
    record_id: Thing,
    event: String,
    content: Value,
    created_at: Datetime,
}

pub struct Syncer {}

impl Syncer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn start_syncing(&self) {
        debug!(target: LOG_TARGET, "Connecting to SurrealDB");
        let surreal: Surreal<surrealdb::engine::any::Any> = Surreal::init();

        // Connect to local SurrealDB instance
        surreal.connect("ws://127.0.0.1:5004/rpc").await.unwrap();
        surreal.use_ns("local").use_db("local").await.unwrap();

        debug!(target: LOG_TARGET, "Spawning syncing task");
        tokio::spawn(async move {
            loop {
                // Run every minute
                sleep(Duration::from_secs(60)).await;

                debug!(target: LOG_TARGET, "Syncing started");

                // Check if local center is a master
                match get_global_key(GlobalKey::InstanceType)
                    .await
                    .unwrap_or_default()
                    .as_str()
                {
                    "master" => {
                        debug!(target: LOG_TARGET, "Local center is a master");
                    }
                    _ => {
                        debug!(target: LOG_TARGET, "Local center is not a master");
                        continue;
                    }
                };

                // Find center id
                debug!(target: LOG_TARGET, "Finding center id");
                let center_id = match get_global_key(GlobalKey::CenterId).await {
                    None => {
                        debug!(target: LOG_TARGET, "No center id set");
                        continue;
                    }
                    Some(center_id) => {
                        debug!(target: LOG_TARGET, "Found center id {}", &center_id);
                        center_id
                    }
                };

                // Find private key
                debug!(target: LOG_TARGET, "Finding private key");
                let private_key = match get_global_key(GlobalKey::PrivateKey).await {
                    None => {
                        debug!(target: LOG_TARGET, "No private key set");
                        continue;
                    }
                    Some(private_key) => private_key,
                };

                // Check if sync is available
                match CentralAPI::check_sync_availability(center_id, private_key).await {
                    Ok(_) => {
                        debug!(target: LOG_TARGET, "Sync is available");
                    }
                    Err(error) => {
                        warn!(target: LOG_TARGET, "Sync is not available: {:?}", error);
                        continue;
                    }
                };

                debug!(target: LOG_TARGET, "Checking if there are changes to push");
                match surreal
                    .query("SELECT * FROM sync WHERE pushed = false LIMIT 100")
                    .await
                {
                    Ok(mut response) => {
                        match response.take::<Vec<SyncEvent>>(0) {
                            Ok(sync_events) => {
                                debug!(target: LOG_TARGET, "Found {} changes to push", sync_events.len());
                            }
                            Err(error) => {
                                warn!(target: LOG_TARGET, "Error parsing changes: {:?}", error);
                            }
                        };
                    }
                    Err(err) => {
                        warn!(target: LOG_TARGET, "Error querying changes: {:?}", err);
                        continue;
                    }
                };
            }
        });
    }
}
