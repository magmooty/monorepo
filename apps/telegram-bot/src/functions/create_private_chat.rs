use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct CreatePrivateChat {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    user_id: i64,

    force: bool,
}

impl CreatePrivateChat {
    pub fn new(client: &TelegramClient, user_id: i64) -> Self {
        Self {
            td_type: TdLibType::CreatePrivateChat,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            user_id,
            force: false,
        }
    }
}
