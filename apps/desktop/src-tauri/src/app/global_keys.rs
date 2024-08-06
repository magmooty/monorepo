use log::{debug, info};
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};

static DB: tokio::sync::OnceCell<SqlitePool> = tokio::sync::OnceCell::const_new();
static DB_URL: &str = "sqlite://global_keys.db";

static LOG_TARGET: &str = "global_keys";

pub async fn init_global_keys() {
    info!(target: LOG_TARGET, "Checking if global_keys.db exists");
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!(target: LOG_TARGET, "Creating global_keys.db");

        match Sqlite::create_database(DB_URL).await {
            Ok(_) => {
                info!(target: LOG_TARGET, "Successfully created global_keys.db");
            }
            Err(error) => panic!("Failed to create global_keys.db: {}", error),
        }
    } else {
        info!(target: LOG_TARGET, "Database global_keys.db exists");
    }

    info!(target: LOG_TARGET, "Openning global_keys.db");
    let pool = SqlitePool::connect("sqlite://global_keys.db")
        .await
        .unwrap();

    info!(target: LOG_TARGET, "Ensuring global_keys table");
    sqlx::query("CREATE TABLE IF NOT EXISTS global_keys (key TEXT PRIMARY KEY, value TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    DB.set(pool).unwrap();
}

#[tauri::command]
#[specta::specta]
pub async fn set_global_key(key: String, value: String) -> Result<(), ()> {
    debug!(target: LOG_TARGET, "Unwrapping pool to set global key {key}");
    let pool = DB.get().unwrap();

    info!(target: LOG_TARGET, "Setting global key {key}");
    sqlx::query("INSERT INTO global_keys (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
        .bind(key)
        .bind(value)
        .execute(pool)
        .await
        .map_err(|_| ())?;

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_global_key(key: String) -> Option<String> {
    debug!(target: LOG_TARGET, "Unwrapping pool to fetch global key {key}");
    let pool = DB.get().unwrap();

    info!(target: LOG_TARGET, "Fetching global key {key}");
    let row = sqlx::query("SELECT value FROM global_keys WHERE key = ?")
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap();

    row.get::<String, &str>("value").into()
}
