use std::sync::Arc;

use crate::{app, SIDECARS};
use log::{debug, error, info, warn};
use tauri::api::process::{Command, CommandEvent};
use tokio::sync::{mpsc::Receiver, oneshot::Sender, Mutex};

static LOG_TARGET: &str = "Surreal sidecar";

async fn listen_for_start_log(
    mut tx: Option<Sender<Result<(), ()>>>,
    logs_rx: Arc<Mutex<Receiver<CommandEvent>>>,
) {
    let mut logs_rx = logs_rx.lock().await;

    while let Some(event) = logs_rx.recv().await {
        match event {
            CommandEvent::Stdout(line) | CommandEvent::Stderr(line) => {
                print!("{}", line);

                // Compare every line until the server is started
                if line.contains("Started web server") {
                    if let Some(tx) = tx.take() {
                        tx.send(Ok(())).unwrap();

                        // Break the loop to avoid expensive operations, we'll start the listener again with less logic
                        return;
                    }
                } else if line.contains("ERROR") {
                    if let Some(tx) = tx.take() {
                        tx.send(Err(())).unwrap();

                        // Break the loop to avoid expensive operations, we'll start the listener again with less logic
                        return;
                    }
                }
            }
            CommandEvent::Error(error) => {
                error!("{}", error);
            }
            CommandEvent::Terminated(_) => {
                error!("SurrealDB sidecar terminated");
            }
            _ => {
                warn!("Unhandled SurrealDB sidecar event: {:?}", event);
            }
        }
    }
}

async fn listen_for_log(logs_rx: Arc<Mutex<Receiver<CommandEvent>>>) {
    info!(target: LOG_TARGET, "Listening for SurrealDB sidecar logs");
    let mut logs_rx = logs_rx.lock().await;

    while let Some(event) = logs_rx.recv().await {
        match event {
            CommandEvent::Stdout(line) | CommandEvent::Stderr(line) => {
                print!("{}", line);
            }
            CommandEvent::Error(error) => {
                error!("{}", error);
            }
            CommandEvent::Terminated(_) => {
                panic!("SurrealDB sidecar terminated");
            }
            _ => {
                warn!("Unhandled SurrealDB sidecar event: {:?}", event);
            }
        }
    }
}

pub async fn run_surreal_sidecar() {
    let credentials = app::get_root_database_credentials();

    let (logs_rx, process) = Command::new_sidecar("surreal")
        .expect("Failed to run SurrealDB sidecar")
        .args([
            "start",
            "--log",
            "info",
            "--user",
            credentials.username.as_str(),
            "--pass",
            credentials.password.as_str(),
            "--bind",
            "0.0.0.0:5004",
            "file:rocksdb",
        ])
        .spawn()
        .expect("Failed to run SurrealDB sidecar");

    SIDECARS.lock().unwrap().push(process);

    let (tx, rx) = tokio::sync::oneshot::channel::<Result<(), ()>>();

    let tx = Some(tx);
    let logs_rx = Arc::new(Mutex::new(logs_rx));

    let start_log_logs_rx = logs_rx.clone();

    tokio::spawn(async move {
        listen_for_start_log(tx, start_log_logs_rx).await;
    });

    debug!(target: LOG_TARGET, "Waiting for SurrealDB sidecar to start");
    match rx.await {
        Ok(Ok(())) => {
            info!(target: LOG_TARGET, "SurrealDB sidecar started successfully");
            tokio::spawn(async move {
                listen_for_log(logs_rx.clone()).await;
            });
        }
        Ok(Err(())) | Err(_) => {
            panic!("Failed to start SurrealDB sidecar");
        }
    };
}
