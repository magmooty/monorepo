use bytes::Bytes;
use database::define_database;
use dotenv::dotenv;
use env_logger::Env;
use http::{header, Response, StatusCode};
use http_body_util::Full;
use log::{debug, info};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use settings::AppSettings;
use std::any::Any;
use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

pub mod app;
pub mod database;
pub mod settings;
pub mod validation;
pub mod whatsapp;

static APP_SETTINGS: OnceCell<AppSettings> = OnceCell::new();
static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

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
    whatsapp::initialize_whatsapp();

    debug!("Parsing environment variables");
    dotenv().ok();
    APP_SETTINGS.get_or_init(|| settings::extract_settings());

    debug!("Connecting to the database");
    let connection = DB
        .connect::<Ws>(&APP_SETTINGS.get().unwrap().surrealdb_endpoint)
        .await;

    match connection {
        Ok(_) => {
            DB.use_ns("magmooty").use_db("magmooty").await.unwrap();
            DB.signin(Root {
                username: "root",
                password: "root",
            })
            .await
            .unwrap();
            info!("Connected to the database");
        }
        Err(e) => {
            panic!("Failed to connect to the database: {}", e);
        }
    }

    define_database().await;

    debug!("Building panic catcher");
    let svc = ServiceBuilder::new()
        // Use `handle_panic` to create the response.
        .layer(CatchPanicLayer::custom(handle_panic));

    debug!("Building app routes");
    let app = app::get_router().layer(svc);

    debug!("Initialize Tokio TCP listener");
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{}", APP_SETTINGS.get().unwrap().port))
            .await
            .unwrap();

    info!("Listening on port {}", APP_SETTINGS.get().unwrap().port);
    axum::serve(listener, app).await.unwrap();
}
