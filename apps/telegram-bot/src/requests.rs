use super::tdlib::ClientId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TdLibType {
    SendMessage,
    UpdateOption,
    Message,
    UpdateMessageSendSucceeded,
    UpdateMessageSendFailed,
    UpdateMessageSendAcknowledged,
    UpdateSavedMessagesTopic,
    InputMessageText,
    FormattedText,
    UpdateAuthorizationState,
    GetAuthorizationState,
    SetLogVerbosityLevel,
    AuthorizationStateWaitTdlibParameters,
    UpdateDefaultBackground,
    UpdateFileDownloads,
    UpdateConnectionState,
    SetTdlibParameters,
    UpdateAnimationSearchParameters,
    SendPhoneNumberCode,
    SetAuthenticationPhoneNumber,
    RequestQrCodeAuthentication,
    CheckAuthenticationCode,
    CheckAuthenticationPassword,
    UpdateAccentColors,
    UpdateProfileAccentColors,
    UpdateSpeechRecognitionTrial,
    UpdateAttachmentMenuBots,
    UpdateDiceEmojis,
    UpdateActiveEmojiReactions,
    UpdateAvailableMessageEffects,
    UpdateChatThemes,
    UpdateReactionNotificationSettings,
    UpdateChatFolders,
    UpdateStoryStealthMode,
    UpdateHavePendingNotifications,
    UpdateUser,
    UpdateChatRemovedFromList,
    UpdateScopeNotificationSettings,
    UpdateUserStatus,
    UpdateSupergroup,
    UpdateBasicGroup,
    UpdateNewChat,
    UpdateChatNotificationSettings,
    UpdateChatLastMessage,
    UpdateChatReadInbox,
    UpdateChatReadOutbox,
    UpdateChatAddedToList,
    UpdateChatMessageAutoDeleteTime,
    UpdateChatPosition,
    UpdateUserFullInfo,
    UpdateChatIsTranslatable,
    UpdateChatAvailableReactions,
    UpdateChatVideoChat,
    UpdateMessageInteractionInfo,
    UpdateSupergroupFullInfo,
    UpdateDefaultReactionType,
    InternalLinkTypeQrCodeAuthentication,
    UpdateNewMessage,
    SearchUserByPhoneNumber,
    SearchContacts,
    CreatePrivateChat,
    UpdateChatActiveStories,
    UpdateGroupCall,
    UpdateChatTheme,
    UpdateChatBackground,
    UpdateUnconfirmedSession,
    UpdateContactCloseBirthdays,
    UpdateDeleteMessages,
    UpdateMessageContent,
    UpdateMessageEdited,
    UpdateInstalledStickerSets,
    UpdateRecentStickers,
    UpdateChatPhoto,
    UpdateChatMessageSender,
    UpdateSuggestedActions,
    UpdateMessageContentOpened,
    Users,
    User,
    Chats,
    Chat,
    Error,
    Ok,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AuthorizationState {
    AuthorizationStateWaitTdlibParameters,
    AuthorizationStateWaitPhoneNumber,
    AuthorizationStateWaitEmailAddress,
    AuthorizationStateWaitEmailCode,
    AuthorizationStateWaitCode,
    AuthorizationStateWaitOtherDeviceConfirmation,
    AuthorizationStateWaitRegistration,
    AuthorizationStateWaitPassword,
    AuthorizationStateReady,
    AuthorizationStateLoggingOut,
    AuthorizationStateClosing,
    AuthorizationStateClosed,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationStateObject {
    #[serde(rename = "@type")]
    pub state: AuthorizationState,

    pub link: Option<String>,

    pub password_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConnectionState {
    #[serde(rename = "connectionStateWaitingForNetwork")]
    ConnectionStateWaitingForNetwork,

    #[serde(rename = "connectionStateConnectingToProxy")]
    ConnectionStateConnectingToProxy,

    #[serde(rename = "connectionStateConnecting")]
    ConnectionStateConnecting,

    #[serde(rename = "connectionStateUpdating")]
    ConnectionStateUpdating,

    #[serde(rename = "connectionStateReady")]
    ConnectionStateReady,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionStateObject {
    #[serde(rename = "@type")]
    pub state: ConnectionState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TDLibResponse {
    #[serde(rename = "@type")]
    pub td_type: TdLibType,

    #[serde(rename = "@client_id")]
    client_id: ClientId,

    #[serde(rename = "@extra")]
    pub extra: Option<String>,

    pub authorization_state: Option<AuthorizationStateObject>,

    pub state: Option<ConnectionStateObject>,

    #[serde(flatten)]
    pub data: serde_json::Value,
}

pub trait TelegramRequest: Serialize {
    fn extra(&self) -> String;
}
