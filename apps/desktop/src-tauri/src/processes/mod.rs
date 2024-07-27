use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::sync::watch;
use tokio::task;

pub async fn run_managed_process(
    command: &str,
    args: &[&str],
    mut shutdown_rx: watch::Receiver<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let stdout_reader = BufReader::new(stdout).lines();
    let stderr_reader = BufReader::new(stderr).lines();

    let stdout_task = task::spawn(async move {
        let mut stdout_lines = stdout_reader;
        while let Some(line) = stdout_lines.next_line().await.unwrap() {
            println!("stdout: {}", line);
        }
    });

    let stderr_task = task::spawn(async move {
        let mut stderr_lines = stderr_reader;
        while let Some(line) = stderr_lines.next_line().await.unwrap() {
            eprintln!("stderr: {}", line);
        }
    });

    tokio::select! {
        _ = shutdown_rx.changed() => {
            println!("Shutdown signal received. Terminating process.");
            child.kill().await?;
        }
        status = child.wait() => {
            println!("Process has exited with status: {:?}", status);
        }
        _ = stdout_task => {},
        _ = stderr_task => {},
    }

    Ok(())
}
