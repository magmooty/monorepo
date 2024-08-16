#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum_test::TestServer;
    use mockall::predicate;
    use serde_json::json;
    use serde_variant::to_variant_name;
    use serial_test::serial;
    use surrealdb::opt::PatchOp;
    use telegram_bot::TelegramClient;

    use crate::{
        app::{
            auth::{
                get_router, ResendSigninCodePayload, ResendSigninCodeStatus, SendSigninCodePayload,
                SendSigninCodeStatus,
            }, common::MessagingChannel, AppState
        },
        database::{Database, SigninCode},
        whatsapp::{MockWhatsAppBot, WASendMessageResponse, WhatsAppStatus},
    };

    async fn setup() -> (Arc<Database>, TestServer) {
        let db = Arc::new(Database::in_memory().await);
        let telegram = TelegramClient::for_testing();
        let state = Arc::new(AppState {
            db: db.clone(),
            telegram,
        });
        let router = get_router().with_state(state).into_make_service();

        (db.clone(), TestServer::new(router).unwrap())
    }

    #[tokio::test]
    #[serial]
    async fn test_resend_signin_code() {
        let ctx = MockWhatsAppBot::send_message_context();

        ctx.expect()
            .with(
                predicate::eq("+201096707442".to_string()),
                predicate::always(),
            )
            .times(2)
            .returning(|_, _| WASendMessageResponse {
                status: WhatsAppStatus::MessageSent,
                error_message: "".to_string(),
            });

        let (_, server) = setup().await;

        let payload = SendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
            channel: MessagingChannel::WhatsApp,
        };

        let response = server.post("/send_signin_code").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&SendSigninCodeStatus::MessageSent).unwrap()
            }
        ));

        let payload = ResendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
            channel: MessagingChannel::WhatsApp,
        };

        let response = server.post("/resend_signin_code").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&ResendSigninCodeStatus::MessageSent).unwrap()
            }
        ));
    }

    #[tokio::test]
    #[serial]
    async fn test_resend_signin_expired_code() {
        let ctx = MockWhatsAppBot::send_message_context();

        ctx.expect()
            .with(
                predicate::eq("+201096707442".to_string()),
                predicate::always(),
            )
            .times(2)
            .returning(|_, _| WASendMessageResponse {
                status: WhatsAppStatus::MessageSent,
                error_message: "".to_string(),
            });

        let (db, server) = setup().await;

        let payload = SendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
            channel: MessagingChannel::WhatsApp,
        };

        let response = server.post("/send_signin_code").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&SendSigninCodeStatus::MessageSent).unwrap()
            }
        ));

        let _: Vec<SigninCode> = db
            .surreal
            .update("signin_code")
            .patch(PatchOp::replace(
                "created_at",
                chrono::Utc::now() - chrono::Duration::minutes(11),
            ))
            .await
            .unwrap();

        let payload = ResendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
            channel: MessagingChannel::WhatsApp,
        };

        let response = server.post("/resend_signin_code").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&ResendSigninCodeStatus::MessageSent).unwrap()
            }
        ));
    }

    #[tokio::test]
    #[serial]
    async fn test_resend_signin_code_no_user() {
        let ctx = MockWhatsAppBot::send_message_context();

        ctx.expect().times(0);

        let (_, server) = setup().await;

        let payload = ResendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
            channel: MessagingChannel::WhatsApp,
        };

        let response = server.post("/resend_signin_code").json(&payload).await;

        response.assert_status_not_found();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&ResendSigninCodeStatus::UserNotFound).unwrap()
            }
        ));
    }
}
