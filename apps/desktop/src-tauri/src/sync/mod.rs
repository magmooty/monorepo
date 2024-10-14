use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::engine::any::Any;
use surrealdb::sql::{Datetime, Thing};
use surrealdb::Surreal;
use tauri::Window;
use tokio::time::{sleep, Duration};

use crate::app::{get_global_key, GlobalKey};
use crate::central::{CentralAPI, CheckSyncAvailabilityError, SyncUploadChunkError};

mod test_sync;

static LOG_TARGET: &str = "Sync";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncEvent {
    id: Thing,
    record_id: Thing,
    event: String,
    content: Value,
    created_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountResponse {
    pub count: i64,
}

pub struct Syncer {}

impl Syncer {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn count_sync_events(surreal: &Surreal<Any>) -> Result<i64, surrealdb::Error> {
        let mut response = surreal
            .query("SELECT COUNT() FROM sync WHERE pushed = false GROUP ALL")
            .await?;

        let count = response.take::<Vec<CountResponse>>(0)?;

        let count = count.get(0).map(|c| c.count).unwrap_or(0);

        info!(target: LOG_TARGET, "Found {} un-synced events", count);

        Ok(count)
    }

    pub async fn fetch_sync_events(
        surreal: &Surreal<Any>,
    ) -> Result<Vec<SyncEvent>, surrealdb::Error> {
        let mut response = surreal
            .query("SELECT * FROM sync WHERE pushed = false LIMIT 100")
            .await?;

        let sync_events = response.take::<Vec<SyncEvent>>(0)?;

        Ok(sync_events)
    }

    pub async fn mark_sync_events_as_pushed(
        surreal: &Surreal<Any>,
        sync_events: &[SyncEvent],
    ) -> Result<(), surrealdb::Error> {
        for sync_event in sync_events {
            surreal
                .update::<Option<Record>>(sync_event.id.clone())
                .merge(json!({"pushed": true}))
                .await?;
        }

        Ok(())
    }

    pub async fn start_syncing(&self, window: Window, test_surreal: Option<Surreal<Any>>) {
        debug!(target: LOG_TARGET, "Connecting to SurrealDB");

        let surreal: Surreal<surrealdb::engine::any::Any>;

        if let Some(test_surreal) = test_surreal {
            surreal = test_surreal;
        } else {
            surreal = Surreal::init();

            // Connect to local SurrealDB instance
            surreal.connect("ws://127.0.0.1:5004/rpc").await.unwrap();
            surreal.use_ns("local").use_db("local").await.unwrap();
        }

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

                while Self::count_sync_events(&surreal).await.unwrap_or_default() > 0 {
                    debug!(target: LOG_TARGET, "Checking if there are changes to push");
                    window
                        .emit("sync_collecting_changes", "")
                        .unwrap_or_default();

                    let sync_events = match Self::fetch_sync_events(&surreal).await {
                        Ok(sync_events) => {
                            info!(target: LOG_TARGET, "Found {} changes to push", sync_events.len());
                            sync_events
                        }
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

                        debug!(target: LOG_TARGET, "Uploading chunk. {}/{} uploaded", uploaded, chunk.len());
                        match CentralAPI::sync_upload_chunk(chunk, &private_key, &center_id).await {
                            Ok(_) => {
                                debug!(target: LOG_TARGET, "Chunk uploaded. {}/{} uploaded", uploaded, chunk.len());

                                debug!(target: LOG_TARGET, "Marking sync events as uploaded");
                                match Self::mark_sync_events_as_pushed(&surreal, &chunk).await {
                                    Ok(_) => {
                                        debug!(target: LOG_TARGET, "Sync events marked as uploaded");
                                        window.emit("sync_progress", uploaded).unwrap_or_default();
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
                            Err(error) => {
                                debug!(target: LOG_TARGET, "Sync failed with error: {:?}", error);
                                window
                                    .emit(
                                        "sync_upload_chunk_failed",
                                        serde_json::to_string(&error).unwrap_or(
                                            serde_json::to_string(
                                                &SyncUploadChunkError::UnknownError,
                                            )
                                            .unwrap(),
                                        ),
                                    )
                                    .unwrap_or_default();
                                continue;
                            }
                        };
                    }
                }

                window.emit("sync_sleep", 60).unwrap_or_default();
            }
        });
    }
}
