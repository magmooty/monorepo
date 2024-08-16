use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct SearchUserByPhoneNumber {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    phone_number: String,

    only_local: bool,
}

impl SearchUserByPhoneNumber {
    pub fn new(client: &TelegramClient, phone_number: String) -> Self {
        Self {
            td_type: TdLibType::SearchUserByPhoneNumber,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            phone_number,
            only_local: false
        }
    }
}
