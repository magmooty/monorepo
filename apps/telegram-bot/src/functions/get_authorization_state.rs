use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAuthorizationState {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    new_verbosity_level: i32,
}

impl GetAuthorizationState {
    pub fn new(client: &TelegramClient, new_verbosity_level: i32) -> Self {
        Self {
            td_type: TdLibType::GetAuthorizationState,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            new_verbosity_level,
        }
    }
}
