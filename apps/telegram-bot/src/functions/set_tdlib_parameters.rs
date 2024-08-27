use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct SetTdLibParameters {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    use_test_dc: bool,

    database_directory: String,

    files_directory: String,

    use_file_database: bool,

    use_chat_info_database: bool,

    use_message_database: bool,

    use_secret_chats: bool,

    api_id: i32,

    api_hash: String,

    system_language_code: String,

    device_model: String,

    system_version: String,

    application_version: String,

    enable_storage_optimizer: bool,

    ignore_file_names: bool,
}

impl SetTdLibParameters {
    pub fn new(client: &TelegramClient, api_id: i32, api_hash: String) -> Self {
        // Create directory third_party if it doesn't exist
        fs::create_dir_all("third_party").unwrap();

        Self {
            td_type: TdLibType::SetTdlibParameters,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            use_test_dc: false,
            database_directory: "third_party/td".to_string(),
            files_directory: "third_party/td_files".to_string(),
            use_file_database: true,
            use_chat_info_database: true,
            use_message_database: false,
            use_secret_chats: false,
            api_id,
            api_hash,
            system_language_code: "en".to_string(),
            device_model: "Magmooty".to_string(),
            system_version: String::default(),
            application_version: "v1.0.0".to_string(),
            enable_storage_optimizer: true,
            ignore_file_names: false,
        }
    }
}
