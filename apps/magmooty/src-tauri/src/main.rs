// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod whatsapp;

#[tauri::command]
async fn test_db() -> String {
    let info = whatsapp::get_info();
    println!("Connection status: {}", info.connection_status);

    let db = database::create_database().await;
    match db {
        Ok(_) => "Database created successfully".to_string(),
        Err(e) => format!("Error creating database: {}", e),
    }
}

#[tokio::main]
async fn main() {
    whatsapp::initialize_whatsapp();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![test_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
