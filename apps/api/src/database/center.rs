use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

use surrealdb::{engine::any::Any, sql::Thing, Surreal};

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub line1: String,
    pub landmark: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Center {
    pub name: String,
    pub public_key: String,
    pub address: Address,
    pub owner: Thing,
}

#[derive(Clone)]
pub struct CenterRepository {
    db: Arc<Surreal<Any>>,
}

impl CenterRepository {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    pub async fn get_center(&self, id: &String) -> Option<Center> {
        let id = Thing::from_str(id).ok()?;

        let center: Result<Option<Center>, surrealdb::Error> = self.db.select(id).await;

        match center {
            Ok(center) => center,
            Err(err) => {
                dbg!(err);
                None
            }
        }
    }
}
