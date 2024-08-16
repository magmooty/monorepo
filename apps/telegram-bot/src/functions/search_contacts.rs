use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct SearchContacts {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    query: String,

    limit: i32,
}

impl SearchContacts {
    pub fn new(client: &TelegramClient, query: String) -> Self {
        Self {
            td_type: TdLibType::SearchContacts,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            query,
            limit: 10
        }
    }
}
