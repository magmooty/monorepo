use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct RootDatabaseCredentials {
    pub username: String,
    pub password: String,
}

#[tauri::command]
#[specta::specta]
pub fn get_root_database_credentials() -> RootDatabaseCredentials {
    RootDatabaseCredentials {
        username: "magmooty".to_string(),
        password: "magmooty".to_string(),
    }
}
