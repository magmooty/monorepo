use crate::tdlib::ClientId;
use crate::{requests::TdLibType, TelegramClient, TelegramRequest};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct FormattedText {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    text: String,
}

impl FormattedText {
    pub fn new(text: String) -> Self {
        Self {
            td_type: TdLibType::FormattedText,
            text,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct InputMessageText {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    text: FormattedText,
}

impl InputMessageText {
    pub fn new(text: FormattedText) -> Self {
        Self {
            td_type: TdLibType::InputMessageText,
            text,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputMessageText),
}

#[derive(Serialize, Debug, Clone, TelegramRequest)]
pub struct SendMessage {
    #[serde(rename = "@type")]
    td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    extra: String,

    chat_id: i64,

    input_message_content: InputMessageContent,
}

impl SendMessage {
    pub fn new(client: &TelegramClient, chat_id: i64, message: String) -> Self {
        Self {
            td_type: TdLibType::SendMessage,
            client_id: client.client_id,
            extra: client.generate_extra_handle(),
            chat_id,
            input_message_content: InputMessageContent::Text(InputMessageText::new(
                FormattedText::new(message),
            )),
        }
    }
}
