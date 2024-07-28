use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::DB;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SigninCode {
    pub phone_number: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone_number: String,
}

pub async fn define_database() {
    debug!("Defining the database schema");

    DB.query(
        "
        # Sign in code table
        DEFINE TABLE signin_code SCHEMAFULL;
        DEFINE FIELD phone_number ON TABLE signin_code TYPE string;
        DEFINE FIELD code ON TABLE signin_code TYPE string;
        DEFINE FIELD created_at ON TABLE signin_code TYPE datetime DEFAULT time::now();
        DEFINE INDEX signin_code_index ON TABLE signin_code COLUMNS code UNIQUE;

        # User table
        DEFINE TABLE user SCHEMALESS;
        DEFINE FIELD first_name ON TABLE user TYPE option<string>;
        DEFINE FIELD last_name ON TABLE user TYPE option<string>;
        DEFINE FIELD phone_number ON TABLE user TYPE option<string>;
        DEFINE FIELD signin_code ON TABLE user TYPE option<record<signin_code>>;
        DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
        DEFINE INDEX phone_number_index ON TABLE user COLUMNS phone_number UNIQUE;

        # Tutor scope, sign in code is valid for 10 minutes
        DEFINE SCOPE tutor SESSION 24h
        SIGNIN ( SELECT * FROM user WHERE phone_number = $phone_number AND signin_code.code = $code AND time::millis(time::now()) - time::millis(signin_code.created_at) < 600000 );
        ",
    )
    .await
    .unwrap();
}
