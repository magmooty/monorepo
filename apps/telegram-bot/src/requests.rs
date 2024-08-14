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

    #[serde(rename = "setLogVerbosityLevel")]
    SetLogVerbosityLevel,

    #[serde(rename = "authorizationStateWaitTdlibParameters")]
    AuthorizationStateWaitTdlibParameters,

    #[serde(rename = "updateDefaultBackground")]
    UpdateDefaultBackground,

    #[serde(rename = "updateFileDownloads")]
    UpdateFileDownloads,

    #[serde(rename = "updateConnectionState")]
    UpdateConnectionState,

    #[serde(rename = "setTdlibParameters")]
    SetTdLibParameters,

    #[serde(rename = "updateAnimationSearchParameters")]
    UpdateAnimationSearchParameters,

    #[serde(rename = "sendPhoneNumberCode")]
    SendPhoneNumberCode,

    #[serde(rename = "setAuthenticationPhoneNumber")]
    SetAuthenticationPhoneNumber,

    #[serde(rename = "requestQrCodeAuthentication")]
    RequestQrCodeAuthentication,

    #[serde(rename = "checkAuthenticationCode")]
    CheckAuthenticationCode,

    #[serde(rename = "checkAuthenticationPassword")]
    CheckAuthenticationPassword,

    #[serde(rename = "updateAccentColors")]
    UpdateAccentColors,

    #[serde(rename = "updateProfileAccentColors")]
    UpdateProfileAccentColors,

    #[serde(rename = "updateSpeechRecognitionTrial")]
    UpdateSpeechRecognitionTrial,

    #[serde(rename = "updateAttachmentMenuBots")]
    UpdateAttachmentMenuBots,

    #[serde(rename = "updateDiceEmojis")]
    UpdateDiceEmojis,

    #[serde(rename = "updateActiveEmojiReactions")]
    UpdateActiveEmojiReactions,

    #[serde(rename = "updateAvailableMessageEffects")]
    UpdateAvailableMessageEffects,

    #[serde(rename = "updateChatThemes")]
    UpdateChatThemes,

    #[serde(rename = "updateReactionNotificationSettings")]
    UpdateReactionNotificationSettings,

    #[serde(rename = "updateChatFolders")]
    UpdateChatFolders,

    #[serde(rename = "updateStoryStealthMode")]
    UpdateStoryStealthMode,

    #[serde(rename = "updateHavePendingNotifications")]
    UpdateHavePendingNotifications,

    #[serde(rename = "updateUser")]
    UpdateUser,

    #[serde(rename = "updateScopeNotificationSettings")]
    UpdateScopeNotificationSettings,

    #[serde(rename = "updateUserStatus")]
    UpdateUserStatus,

    #[serde(rename = "updateSupergroup")]
    UpdateSupergroup,

    #[serde(rename = "updateBasicGroup")]
    UpdateBasicGroup,

    #[serde(rename = "updateNewChat")]
    UpdateNewChat,

    #[serde(rename = "updateChatNotificationSettings")]
    UpdateChatNotificationSettings,

    #[serde(rename = "updateChatLastMessage")]
    UpdateChatLastMessage,

    #[serde(rename = "updateChatReadInbox")]
    UpdateChatReadInbox,

    #[serde(rename = "updateChatReadOutbox")]
    UpdateChatReadOutbox,

    #[serde(rename = "updateChatAddedToList")]
    UpdateChatAddedToList,

    #[serde(rename = "updateChatMessageAutoDeleteTime")]
    UpdateChatMessageAutoDeleteTime,

    #[serde(rename = "updateChatPosition")]
    UpdateChatPosition,

    #[serde(rename = "updateUserFullInfo")]
    UpdateUserFullInfo,

    #[serde(rename = "updateChatIsTranslatable")]
    UpdateChatIsTranslatable,

    #[serde(rename = "updateChatAvailableReactions")]
    UpdateChatAvailableReactions,

    #[serde(rename = "updateChatVideoChat")]
    UpdateChatVideoChat,

    #[serde(rename = "updateMessageInteractionInfo")]
    UpdateMessageInteractionInfo,

    #[serde(rename = "updateSupergroupFullInfo")]
    UpdateSupergroupFullInfo,

    #[serde(rename = "updateDefaultReactionType")]
    UpdateDefaultReactionType,

    #[serde(rename = "internalLinkTypeQrCodeAuthentication")]
    InternalLinkTypeQrCodeAuthentication,

    #[serde(rename = "error")]
    Error,

    #[serde(rename = "ok")]
    Ok,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AuthorizationState {
    #[serde(rename = "authorizationStateWaitTdlibParameters")]
    AuthorizationStateWaitTdlibParameters,

    #[serde(rename = "authorizationStateWaitPhoneNumber")]
    AuthorizationStateWaitPhoneNumber,

    #[serde(rename = "authorizationStateWaitCode")]
    AuthorizationStateWaitCode,

    #[serde(rename = "authorizationStateWaitPassword")]
    AuthorizationStateWaitPassword,

    #[serde(rename = "authorizationStateWaitOtherDeviceConfirmation")]
    AuthorizationStateWaitOtherDeviceConfirmation,

    #[serde(rename = "authorizationStateReady")]
    AuthorizationStateReady,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthorizationStateObject {
    #[serde(rename = "@type")]
    pub state: AuthorizationState,

    pub link: Option<String>,

    pub password_hint: Option<String>,
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
}

pub trait TelegramRequest: Serialize {
    fn extra(&self) -> String;
}
