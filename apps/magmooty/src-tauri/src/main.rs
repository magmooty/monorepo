// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use std::sync::Arc;
use surrealdb::{engine::local::Db, Surreal};
use tauri_specta::ts;

mod app;
mod database;
mod whatsapp;

struct AppState {
    database: Arc<Surreal<Db>>,
}

#[tokio::main]
async fn main() {
    whatsapp::initialize_whatsapp();

    let database = database::create_database().await.unwrap();

    let state = AppState {
        database: Arc::new(database),
    };

    ts::export(
        collect_types![
            app::whatsapp::whatsapp_get_info,
            app::whatsapp::whatsapp_start_connection,
            app::whatsapp::whatsapp_send_message,
        ],
        "../src/lib/bindings.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::whatsapp::whatsapp_get_info,
            app::whatsapp::whatsapp_start_connection,
            app::whatsapp::whatsapp_send_message,
        ])
        .manage(state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
