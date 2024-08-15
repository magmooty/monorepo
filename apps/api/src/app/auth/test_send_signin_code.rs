#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum_test::TestServer;
    use mockall::predicate;
    use serde_json::json;
    use serde_variant::to_variant_name;
    use serial_test::serial;

    use crate::{
        app::{
            auth::{get_router, SendSigninCodePayload, SendSigninCodeStatus},
            AppState,
        },
        database::Database,
        whatsapp::{MockWhatsAppBot, WASendMessageResponse, WhatsAppStatus},
    };

    async fn setup() -> (Arc<Database>, TestServer) {
        let db = Arc::new(Database::in_memory().await);
        let state = Arc::new(AppState { db: db.clone() });
        let router = get_router().with_state(state).into_make_service();

        (db.clone(), TestServer::new(router).unwrap())
    }

    #[tokio::test]
    #[serial]
    async fn test_send_signin_code_new_user_creation() {
        let ctx = MockWhatsAppBot::send_message_context();

        ctx.expect()
            .with(
                predicate::eq("+201096707442".to_string()),
                predicate::always(),
            )
            .times(1)
            .returning(|_, _| WASendMessageResponse {
                status: WhatsAppStatus::MessageSent,
                error_message: "".to_string(),
            });

        let (db, server) = setup().await;

        let payload = SendSigninCodePayload {
            phone_number: "+201096707442".to_string(),
        };

        let response = server.post("/send_signin_code").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": to_variant_name(&SendSigninCodeStatus::MessageSent).unwrap()
            }
        ));

        let user = db.user.find_user(&"+201096707442".to_string()).await;

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.phone_number, "+201096707442");
    }
}
