use serde::{Deserialize, Serialize};
use std::sync::Arc;

use surrealdb::{engine::any::Any, Surreal};

use super::Record;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: String,
}

#[derive(Clone)]
pub struct UserRepostiory {
    db: Arc<Surreal<Any>>,
}

impl UserRepostiory {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    pub async fn find_user(&self, phone_number: &String) -> Option<User> {
        self.db
            .query("SELECT * FROM user WHERE phone_number = $phone_number")
            .bind(("phone_number", phone_number))
            .await
            .unwrap()
            .take::<Option<User>>(0)
            .unwrap()
    }

    pub async fn create_user(&self, phone_number: &String) -> Record {
        let user: Vec<Record> = self
            .db
            .create("user")
            .content(User {
                first_name: None,
                last_name: None,
                phone_number: phone_number.clone(),
            })
            .await
            .unwrap();

        user.first().take().unwrap().clone()
    }
}
