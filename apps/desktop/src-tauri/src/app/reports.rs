use log::info;

use crate::pdf::{generate_pdf, Report};

static LOG_TARGET: &str = "Report generation";

#[tauri::command]
#[specta::specta]
pub async fn generate_report(data: Report, file_path: String) -> Result<(), String> {
    info!(target: LOG_TARGET, "Generating report to {}", file_path);

    generate_pdf(data, file_path)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
