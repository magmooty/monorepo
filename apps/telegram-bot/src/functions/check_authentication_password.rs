use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct CheckAuthenticationPassword {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    password: String,
}

impl CheckAuthenticationPassword {
    pub fn new(client: &TelegramClient, password: String) -> Self {
        Self {
            td_type: TdLibType::CheckAuthenticationPassword,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            password,
        }
    }
}
