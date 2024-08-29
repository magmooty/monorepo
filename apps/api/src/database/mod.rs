use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;

mod center;
mod signin_code;
mod user;
mod sync;

mod schema;
mod local_structs;

mod test_center;
mod test_signin_code;
mod test_user;

pub use center::*;
pub use signin_code::*;
pub use sync::*;
pub use user::*;

static LOG_TARGET: &str = "Database";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Clone)]
pub struct Database {
    #[cfg(test)]
    pub surreal: Arc<surrealdb::Surreal<surrealdb::engine::any::Any>>,

    #[cfg(not(test))]
    surreal: Arc<surrealdb::Surreal<surrealdb::engine::any::Any>>,

    pub signin_code: SignInCodeRepository,
    pub user: UserRepository,
    pub center: CenterRepository,
    pub sync: SyncRepository,

    endpoint: &'static str,
    credentials: Option<Root<'static>>,
}

impl Database {
    pub fn new(endpoint: &'static str, credentials: Option<Root<'static>>) -> Self {
        let surreal = Arc::new(surrealdb::Surreal::init());
        let signin_code = SignInCodeRepository::new(surreal.clone());
        let user = UserRepository::new(surreal.clone());
        let center = CenterRepository::new(surreal.clone());
        let sync = SyncRepository::new(endpoint, credentials);

        Self {
            surreal,
            signin_code,
            user,
            center,
            sync,
            endpoint,
            credentials,
        }
    }

    pub async fn in_memory() -> Self {
        let db = Database::new("mem://", None);

        db.connect().await;

        db
    }

    pub async fn connect(&self) {
        let connection = self.surreal.connect(self.endpoint).await;

        match connection {
            Ok(_) => {
                self.surreal
                    .use_ns("magmooty")
                    .use_db("magmooty")
                    .await
                    .unwrap();

                if let Some(credentials) = self.credentials {
                    self.surreal.signin(credentials).await.unwrap();
                }

                info!(target: LOG_TARGET, "Connected to the database");
            }
            Err(e) => {
                panic!("Failed to connect to the database: {}", e);
            }
        }
    }

    pub async fn define_database(&self) {
        debug!(target: LOG_TARGET, "Defining the database schema");

        self.surreal
            .use_ns("magmooty")
            .use_db("magmooty")
            .await
            .unwrap();

        self.surreal.query(
            "
            # Sign in code table
            DEFINE TABLE signin_code SCHEMAFULL;
            DEFINE FIELD phone_number ON TABLE signin_code TYPE string;
            DEFINE FIELD code ON TABLE signin_code TYPE string;
            DEFINE FIELD created_at ON TABLE signin_code TYPE datetime DEFAULT time::now();
            DEFINE INDEX signin_code_index ON TABLE signin_code COLUMNS code UNIQUE;
    
            # User table
            DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR SELECT WHERE id = $auth.id;
            DEFINE FIELD first_name ON TABLE user TYPE option<string>;
            DEFINE FIELD last_name ON TABLE user TYPE option<string>;
            DEFINE FIELD phone_number ON TABLE user TYPE option<string>;
            DEFINE FIELD signin_code ON TABLE user TYPE option<record<signin_code>>;
            DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
            DEFINE INDEX phone_number_index ON TABLE user COLUMNS phone_number UNIQUE;
    
            DEFINE TABLE center SCHEMALESS PERMISSIONS FOR SELECT, CREATE, UPDATE WHERE owner = $auth.id;
            DEFINE FIELD name ON TABLE center TYPE string;
            DEFINE FIELD address ON TABLE center FLEXIBLE TYPE object;
            DEFINE FIELD public_key ON TABLE center TYPE string;
            DEFINE FIELD owner ON TABLE center TYPE record<user>;
    
            # Tutor scope, sign in code is valid for 10 minutes
            DEFINE SCOPE tutor SESSION 24h
            SIGNIN ( SELECT * FROM user WHERE phone_number = $phone_number AND signin_code.code = $code AND time::millis(time::now()) - time::millis(signin_code.created_at) < 600000 );
            ",
        )
        .await
        .unwrap();
    }

    #[cfg(test)]
    pub async fn clear_database(&self) {
        self.surreal
            .query("REMOVE NAMESPACE magmooty")
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::database::Database;

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_define_database() {
        let db = setup().await;
        db.define_database().await;
    }
}
