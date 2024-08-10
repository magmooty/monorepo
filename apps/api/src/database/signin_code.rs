use rand::Rng;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use surrealdb::{engine::any::Any, Surreal};

use super::Record;

#[derive(Deserialize, Serialize, Clone)]
pub struct SigninCode {
    pub phone_number: String,
    pub code: String,
}

#[derive(Clone)]
pub struct SignInCodeRepository {
    db: Arc<Surreal<Any>>,
}

impl SignInCodeRepository {
    pub fn new(db: Arc<Surreal<Any>>) -> Self {
        Self { db }
    }

    pub async fn delete_previous_signin_codes(&self, phone_number: &String) -> () {
        self.db
            .query("DELETE signin_code WHERE phone_number = $phone_number")
            .bind(("phone_number", phone_number))
            .await
            .unwrap();
    }

    pub async fn create_signin_code(&self, phone_number: &String, iteration: i32) -> String {
        if iteration > 10 {
            panic!("Failed to create signin code after 10 attempts");
        }

        // Generate a random 8 digit code
        let code = tokio::spawn(async {
            let mut rng = rand::thread_rng();
            rng.gen_range(00000000..=99999999)
        })
        .await
        .unwrap();

        let signin_code_record = self
            .db
            .create::<Vec<Record>>("signin_code")
            .content(SigninCode {
                phone_number: phone_number.clone(),
                code: code.to_string(),
            })
            .await
            .unwrap()
            .first()
            .take()
            .unwrap()
            .clone();

        self.db
            .query("UPDATE user SET signin_code = $code")
            .bind(("code", signin_code_record.id))
            .await
            .unwrap();

        code.to_string()
    }
}
