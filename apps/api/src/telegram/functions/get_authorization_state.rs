use serde::{Deserialize, Serialize};

use crate::telegram::tdlib::ClientId;
use crate::telegram::{requests::TdLibType, TelegramClient, TelegramRequest};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetLogVerbosityLevel {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    new_verbosity_level: i32,
}

impl TelegramRequest for SetLogVerbosityLevel {
    fn extra(&self) -> String {
        self.extra.clone()
    }
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
