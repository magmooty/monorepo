use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};
use telegram_macros::TelegramRequest;

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct SetLogVerbosityLevel {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    new_verbosity_level: i32,
}

impl SetLogVerbosityLevel {
    pub fn new(client: &TelegramClient, new_verbosity_level: i32) -> Self {
        Self {
            td_type: TdLibType::SetLogVerbosityLevel,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            new_verbosity_level,
        }
    }
}
