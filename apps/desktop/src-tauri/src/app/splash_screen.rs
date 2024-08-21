use log::debug;
use tauri::{Manager, Window};

static LOG_TARGET: &str = "Splash Screen";

#[tauri::command]
#[specta::specta]
pub async fn open_splash_screen(window: Window) {
    debug!(target: LOG_TARGET, "Showing splash screen");
    if let Some(window) = window.get_window("splashscreen") {
        window.show().unwrap_or_default();
    }
}

#[tauri::command]
#[specta::specta]
pub async fn close_splash_screen(window: Window) {
    debug!(target: LOG_TARGET, "Closing splash screen");
    if let Some(window) = window.get_window("splashscreen") {
        window.close().unwrap_or_default();
    }

    debug!(target: LOG_TARGET, "Showing main window");
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}
