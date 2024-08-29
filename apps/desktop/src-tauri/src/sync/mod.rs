use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::Surreal;
use tauri::Window;
use tokio::time::{sleep, Duration};

use crate::app::{get_global_key, GlobalKey};
use crate::central::{CentralAPI, CheckSyncAvailabilityError, SyncUploadChunkError};

static LOG_TARGET: &str = "Sync";

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub async fn start_syncing(&self, window: Window) {
        debug!(target: LOG_TARGET, "Connecting to SurrealDB");
        let surreal: Surreal<surrealdb::engine::any::Any> = Surreal::init();

        // Connect to local SurrealDB instance
        surreal.connect("ws://127.0.0.1:5004/rpc").await.unwrap();
        surreal.use_ns("local").use_db("local").await.unwrap();

        debug!(target: LOG_TARGET, "Spawning syncing task");
        tokio::spawn(async move {
            loop {
                // Run every minute
                sleep(Duration::from_secs(5)).await;

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
                match CentralAPI::check_sync_availability(&center_id, &private_key).await {
                    Ok(_) => {
                        debug!(target: LOG_TARGET, "Sync is available");
                        window.emit("sync_available", "").unwrap_or_default();
                    }
                    Err(error) => {
                        warn!(target: LOG_TARGET, "Sync is not available: {:?}", error);
                        window
                            .emit(
                                "sync_unavailable",
                                serde_json::to_string(&error).unwrap_or(
                                    serde_json::to_string(
                                        &CheckSyncAvailabilityError::UnknownError,
                                    )
                                    .unwrap(),
                                ),
                            )
                            .unwrap();
                        continue;
                    }
                };

                debug!(target: LOG_TARGET, "Checking if there are changes to push");
                window
                    .emit("sync_collecting_changes", "")
                    .unwrap_or_default();

                let sync_events = match surreal
                    .query("SELECT * FROM sync WHERE pushed = false LIMIT 100")
                    .await
                {
                    Ok(mut response) => match response.take::<Vec<SyncEvent>>(0) {
                        Ok(sync_events) => {
                            info!(target: LOG_TARGET, "Found {} changes to push", sync_events.len());
                            sync_events
                        }
                        Err(err) => {
                            error!(target: LOG_TARGET, "Error parsing changes: {:?}", err);
                            window
                                .emit("sync_collecting_changes_failed", err.to_string())
                                .unwrap_or_default();
                            continue;
                        }
                    },
                    Err(err) => {
                        error!(target: LOG_TARGET, "Error querying changes: {:?}", err);
                        window
                            .emit("sync_collecting_changes_failed", err.to_string())
                            .unwrap_or_default();
                        continue;
                    }
                };

                window
                    .emit("sync_start", sync_events.len())
                    .unwrap_or_default();

                debug!(target: LOG_TARGET, "Uploading chunks of data");
                let mut uploaded = 0;

                for chunk in sync_events.chunks(100) {
                    uploaded += chunk.len();

                    match CentralAPI::sync_upload_chunk(chunk, &private_key, &center_id).await {
                        Ok(_) => {
                            debug!(target: LOG_TARGET, "Chunk uploaded");
                            window.emit("sync_progress", uploaded).unwrap_or_default();

                            debug!(target: LOG_TARGET, "Marking sync events as uploaded");
                            for sync_event in chunk {
                                match surreal
                                    .query(
                                        format!(
                                            "UPDATE sync SET pushed = true WHERE record_id = '{}'",
                                            sync_event.record_id
                                        )
                                        .as_str(),
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        debug!(target: LOG_TARGET, "Sync event marked as uploaded");
                                    }
                                    Err(err) => {
                                        error!(target: LOG_TARGET, "Error marking sync event as uploaded: {:?}", err);
                                        window
                                            .emit("sync_upload_chunk_failed", err.to_string())
                                            .unwrap_or_default();
                                        continue;
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            debug!(target: LOG_TARGET, "Sync failed with error: {:?}", error);
                            window
                                .emit(
                                    "sync_upload_chunk_failed",
                                    serde_json::to_string(&error).unwrap_or(
                                        serde_json::to_string(&SyncUploadChunkError::UnknownError)
                                            .unwrap(),
                                    ),
                                )
                                .unwrap_or_default();
                            continue;
                        }
                    };
                }

                window.emit("sync_sleep", 60).unwrap_or_default();
            }
        });
    }
}
