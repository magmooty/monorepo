// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use specta::collect_types;
use tauri_specta::ts;

mod app;
mod whatsapp;

use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use std::env;
use std::path::PathBuf;

async fn run_surrealdb(verbose: bool) {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let surreal_db_path: PathBuf = current_dir.join("surreal");

    let mut surrealdb_process = Command::new(surreal_db_path)
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
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    if verbose {
        let stdout = surrealdb_process
            .stdout
            .take()
            .expect("Failed to capture stdout");

        let stderr = surrealdb_process
            .stderr
            .take()
            .expect("Failed to capture stderr");

        let stdout_task = tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                println!("{}", line);
            }
        });

        // Create a task to read stderr
        let stderr_task = tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                eprintln!("{}", line);
            }
        });

        stdout_task.await.expect("Failed to join stdout task");
        stderr_task.await.expect("Failed to join stderr task");
    }
}

#[tokio::main]
async fn main() {
    whatsapp::initialize_whatsapp();

    tokio::task::spawn(run_surrealdb(true));

    ts::export(
        collect_types![
            app::whatsapp::whatsapp_get_info,
            app::whatsapp::whatsapp_start_connection,
            app::whatsapp::whatsapp_send_message,
        ],
        "../src/lib/whatsapp.ts",
    )
    .unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app::whatsapp::whatsapp_get_info,
            app::whatsapp::whatsapp_start_connection,
            app::whatsapp::whatsapp_send_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
