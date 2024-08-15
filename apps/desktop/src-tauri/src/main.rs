// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use log::info;
use panic_handler::initialize_graceful_panic_handler;
use process_killer::kill_hanging_sidecars;
use simple_logger;
use specta::collect_types;
use sync::Syncer;
use tauri::api::process::CommandChild;
use tauri_specta::ts;

mod app;
mod central;
mod network_discovery;
mod panic_handler;
mod process_killer;
mod surreal_sidecar;
mod sync;
mod whatsapp_sidecar;

use surreal_sidecar::run_surreal_sidecar;
use whatsapp_sidecar::run_whatsapp_sidecar;

static LOG_TARGET: &str = "main";

static SIDECARS: once_cell::sync::Lazy<Arc<std::sync::Mutex<Vec<CommandChild>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(std::sync::Mutex::new(Vec::new())));

fn generate_typescript_bindings() {
    ts::export(
        collect_types![
            app::set_global_key,
            app::get_global_key,
            app::generate_key_pair,
            app::discover_network,
            app::get_root_database_credentials,
            app::open_splash_screen,
            app::close_splash_screen
        ],
        "../src/lib/bindings.ts",
    )
    .unwrap();
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    kill_hanging_sidecars();
    initialize_graceful_panic_handler();

    // Only run if in debug (dev) mode
    #[cfg(debug_assertions)]
    {
        info!(target: LOG_TARGET, "Generating TypeScript bindings");
        generate_typescript_bindings();
    }

    info!(target: LOG_TARGET, "Initializing network discovery UDP transceiver");
    tokio::spawn(network_discovery::start_network_discovery_receiver());

    info!(target: LOG_TARGET, "Running local WhatsApp API");
    run_whatsapp_sidecar().await;

    info!(target: LOG_TARGET, "Running local SurrealDB");
    run_surreal_sidecar().await;

    info!(target: LOG_TARGET, "Running syncer");
    let syncer = Syncer::new();
    syncer.start_syncing().await;

    // Run App
    info!(target: LOG_TARGET, "Running main application window");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::set_global_key,
            app::get_global_key,
            app::generate_key_pair,
            app::discover_network,
            app::get_root_database_credentials,
            app::open_splash_screen,
            app::close_splash_screen
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
