use dotenv::dotenv;
use once_cell::sync::OnceCell;
use settings::AppSettings;

pub mod settings;

static app_settings: OnceCell<AppSettings> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv().ok();

    app_settings.get_or_init(|| settings::extract_settings());
}
