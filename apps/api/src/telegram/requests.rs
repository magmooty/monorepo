use super::tdlib::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TdLibType {
    #[serde(rename = "updateOption")]
    UpdateOption,

    #[serde(rename = "updateAuthorizationState")]
    UpdateAuthorizationState,

    #[serde(rename = "getAuthorizationState")]
    GetAuthorizationState,

    #[serde(rename = "authorizationStateWaitTdlibParameters")]
    AuthorizationStateWaitTdlibParameters,

    #[serde(rename = "setLogVerbosityLevel")]
    SetLogVerbosityLevel,

    #[serde(rename = "ok")]
    Ok,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TDLibResponse {
    #[serde(rename = "@type")]
    pub td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    pub extra: Option<String>,
}

pub trait TelegramRequest: Serialize {
    fn extra(&self) -> String;
}
