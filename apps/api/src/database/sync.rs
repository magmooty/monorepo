use super::local_structs::Content;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;

use surrealdb::{
    engine::any::Any,
    opt::auth::Root,
    sql::{Datetime, Thing},
    Surreal,
};

use super::{schema::LOCAL_SCHEMA, Record};

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct SyncEvent {
    record_id: Thing,
    event: String,
    content: Content,
    created_at: Datetime,
}

#[derive(Serialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InsertSyncEventsError {
    InvalidCenterID,
    DatabaseConnectionError,
    DatabaseSchemaSetupError,
    InsertionError(Option<String>),
    UpdateError,
    DeletionError,
    UnknownEvent,
}

#[derive(Clone)]
pub struct SyncRepository {
    endpoint: &'static str,
    credentials: Option<Root<'static>>,
}

impl SyncRepository {
    pub fn new(endpoint: &'static str, credentials: Option<Root<'static>>) -> Self {
        Self {
            endpoint,
            credentials,
        }
    }

    pub async fn insert_sync_events(
        &self,
        center_id: &String,
        events: Vec<SyncEvent>,
    ) -> Result<(), InsertSyncEventsError> {
        let center_id =
            Thing::from_str(center_id).map_err(|_| InsertSyncEventsError::InvalidCenterID)?;

        let db: Surreal<Any> = Surreal::init();

        db.connect(self.endpoint)
            .await
            .map_err(|_| InsertSyncEventsError::DatabaseConnectionError)?;

        db.use_ns("magmooty")
            .use_db(center_id.id.to_string())
            .await
            .map_err(|_| InsertSyncEventsError::DatabaseConnectionError)?;

        let credentials = self
            .credentials
            .ok_or_else(|| InsertSyncEventsError::DatabaseConnectionError)?;

        db.signin(credentials)
            .await
            .map_err(|_| InsertSyncEventsError::DatabaseConnectionError)?;

        db.query(LOCAL_SCHEMA)
            .await
            .map_err(|_| InsertSyncEventsError::DatabaseSchemaSetupError)?;

        for event in events {
            match event.event.as_str() {
                "CREATE" => {
                    let _: Record = db
                        .create(event.record_id)
                        .content(event.content)
                        .await
                        .map_err(|err| {
                            InsertSyncEventsError::InsertionError(Some(err.to_string()))
                        })?
                        .ok_or_else(|| InsertSyncEventsError::InsertionError(None))?;
                }
                "UPDATE" => {
                    let _: Record = db
                        .update(event.record_id)
                        .content(event.content)
                        .await
                        .map_err(|_| InsertSyncEventsError::UpdateError)?
                        .ok_or_else(|| InsertSyncEventsError::UpdateError)?;
                }
                "DELETE" => {
                    let _: Record = db
                        .delete(event.record_id)
                        .await
                        .map_err(|_| InsertSyncEventsError::DeletionError)?
                        .ok_or_else(|| InsertSyncEventsError::DeletionError)?;
                }
                _ => Err(InsertSyncEventsError::UnknownEvent)?,
            }
        }

        Ok(())
    }
}
