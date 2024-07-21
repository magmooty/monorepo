// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;
mod router;
mod whatsapp;
use std::sync::Arc;

struct TauriAppContext {
    application: Arc<router::Router<app::AppResponse, app::AppError>>,
}

#[tokio::main]
async fn main() {
    whatsapp::initialize_whatsapp();
    let application = app::run();
    tauri::Builder::default()
        .manage(TauriAppContext {
            application: Arc::new(application),
        })
        .invoke_handler(tauri::generate_handler![app::query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
