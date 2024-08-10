use bytes::Bytes;
use database::Database;
use dotenv::dotenv;
use env_logger::Env;
use http::{header, Response, StatusCode};
use http_body_util::Full;
use log::{debug, info};
use once_cell::sync::Lazy;
use settings::AppSettings;
use std::any::Any;
use std::sync::Arc;
use surrealdb::opt::auth::Root;
use telegram_bot;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

pub mod app;
pub mod database;
pub mod settings;
pub mod validation;
pub mod whatsapp;

static APP_SETTINGS: Lazy<AppSettings> = Lazy::new(|| {
    debug!("Parsing environment variables");
    dotenv().ok();
    settings::extract_settings()
});

fn handle_panic(err: Box<dyn Any + Send + 'static>) -> Response<Full<Bytes>> {
    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown error occurred".to_string()
    };

    let body = serde_json::json!({
        "error_message": details,
        "details": "panic"
    });

    let body = serde_json::to_string(&body).unwrap();

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Full::from(body))
        .unwrap()
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    debug!("Starting the application");

    debug!("Initializing WhatsApp");
    whatsapp::WhatsAppBot::initialize_whatsapp();

    debug!("Initializing Telegram");
    telegram_bot::initialize_telegram().await;

    debug!("Connecting to the database");
    let database = Arc::new(Database::new());
    database
        .connect(
            &APP_SETTINGS.surrealdb_endpoint,
            Some(Root {
                username: &APP_SETTINGS.surrealdb_root_username,
                password: &APP_SETTINGS.surrealdb_root_password,
            }),
        )
        .await;

    debug!("Defining database schema");
    database.define_database().await;

    debug!("Building panic catcher");
    let svc = ServiceBuilder::new()
        // Use `handle_panic` to create the response.
        .layer(CatchPanicLayer::custom(handle_panic));

    debug!("Building app routes");
    let app = app::create_app_router()
        .layer(svc)
        .with_state(Arc::new(app::AppState {
            db: database.clone(),
        }))
        .into_make_service();

    debug!("Initialize Tokio TCP listener");
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &APP_SETTINGS.port))
        .await
        .unwrap();

    info!("Listening on port {}", &APP_SETTINGS.port);
    axum::serve(listener, app).await.unwrap();
}
