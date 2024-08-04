use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
pub struct AppSettings {
    pub surrealdb_endpoint: String,
    pub port: i32,
    pub admin_public_key: String,
    pub telegram_api_id: String,
    pub telegram_api_hash: String,
}

pub fn extract_settings() -> AppSettings {
    let config = Config::builder()
        .add_source(Environment::default().try_parsing(true))
        .build()
        .unwrap();

    let app_settings: AppSettings = config.try_deserialize().unwrap();

    app_settings
}
