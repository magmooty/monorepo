// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use tauri::api::process::{Command, CommandEvent};
use tauri_specta::ts;

mod app;
mod network_discovery;

fn generate_typescript_bindings() {
    ts::export(
        collect_types![app::check_internet_connection],
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
    generate_typescript_bindings();

    tokio::spawn(network_discovery::start_network_discovery_receiver());

    run_whatsapp_sidecar();
    run_surreal_sidecar();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![app::check_internet_connection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
