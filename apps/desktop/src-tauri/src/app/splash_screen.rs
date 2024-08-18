use log::debug;
use tauri::{Manager, Window};

static LOG_TARGET: &str = "Splash Screen";

#[tauri::command]
#[specta::specta]
pub async fn open_splash_screen(window: Window) {
    debug!(target: LOG_TARGET, "Showing splash screen");
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .show()
        .unwrap_or_default();
}

#[tauri::command]
#[specta::specta]
pub async fn close_splash_screen(window: Window) {
    debug!(target: LOG_TARGET, "Closing splash screen");
    window
        .get_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap_or_default();

    debug!(target: LOG_TARGET, "Showing main window");
    window
        .get_window("main")
        .expect("no window labeled 'main' found")
        .show()
        .unwrap();
}
