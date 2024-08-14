use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, TelegramRequest)]
pub struct RequestQrCodeAuthentication {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,
}

impl RequestQrCodeAuthentication {
    pub fn new(client: &TelegramClient) -> Self {
        Self {
            td_type: TdLibType::RequestQrCodeAuthentication,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
        }
    }
}
