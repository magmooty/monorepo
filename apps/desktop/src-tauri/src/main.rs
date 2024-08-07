// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::init_global_keys;
use log::info;
use simple_logger;
use specta::collect_types;
use tauri::api::process::{Command, CommandEvent};
use tauri_specta::ts;

mod app;
mod network_discovery;

static LOG_TARGET: &str = "main";

fn generate_typescript_bindings() {
    ts::export(
        collect_types![
            app::set_global_key,
            app::get_global_key,
            app::generate_key_pair
        ],
        "../src/lib/bindings.ts",
    )
    .unwrap();
}

fn run_whatsapp_sidecar() {
    let (mut rx, _) = Command::new_sidecar("whatsapp-bot")
        .expect("Failed to run WhatsApp bot sidecar")
        .spawn()
        .expect("Failed to run WhatsApp bot sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(_) = event {
                // print!("{}", line);
            }
        }
    });
}

fn run_surreal_sidecar() {
    let (mut rx, _) = Command::new_sidecar("surreal")
        .expect("Failed to run SurrealDB sidecar")
        .args([
            "start",
            "--log",
            "trace",
            "--user",
            "root",
            "--pass",
            "root",
            "file:rocksdb",
        ])
        .spawn()
        .expect("Failed to run SurrealDB sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(_) = event {
                // print!("{}", line);
            }
        }
    });
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();

    info!(target: LOG_TARGET, "Generating TypeScript bindings");
    generate_typescript_bindings();

    info!(target: LOG_TARGET, "Initializing global keys");
    init_global_keys().await;

    info!(target: LOG_TARGET, "Initializing network discovery UDP transceiver");
    tokio::spawn(network_discovery::start_network_discovery_receiver());

    dbg!(network_discovery::discover_network().await.unwrap());

    info!(target: LOG_TARGET, "Running local WhatsApp API");
    run_whatsapp_sidecar();

    info!(target: LOG_TARGET, "Running local SurrealDB");
    run_surreal_sidecar();

    // Run App
    info!(target: LOG_TARGET, "Running main application window");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::set_global_key,
            app::get_global_key,
            app::generate_key_pair
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
