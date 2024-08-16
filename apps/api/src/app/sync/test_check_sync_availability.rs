#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum_test::TestServer;
    use base64::Engine;
    use jsonwebtoken::{Algorithm, EncodingKey, Header};
    use openssl::rsa::Rsa;
    use serde_json::json;
    use surrealdb::sql::Thing;
    use telegram_bot::TelegramClient;

    use crate::{
        app::{
            sync::{get_router, CheckSyncAvailabilityPayload},
            AppState,
        },
        database::{Address, Center, Database, Record},
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
    async fn test_invalid_jwt_signature() {
        let (_, server) = setup().await;

        let payload = CheckSyncAvailabilityPayload {
            center_id: "center_id".to_string(),
            signature: "signature".to_string(),
        };

        let response = server.post("/check_sync_availability").json(&payload).await;

        response.assert_status_unauthorized();

        response.assert_json(&json!(
            {
                "status": "center_signature_invalid"
            }
        ));
    }

    #[tokio::test]
    async fn test_center_not_found() {
        let (_, server) = setup().await;

        let payload = CheckSyncAvailabilityPayload {
            center_id: "center_id".to_string(),
            signature: "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJjZW50ZXJfaWQiOiJjZW50ZXI6ejB6d3Y2M2lhYXp5cThpZHdqZDgifQ.i4kEOuj_nS7vq7YLN7SceQa3zBjNHXCbOiZ2L6f8GAFUkrMVBGwUhn7D9J1jlQhFXbnJUiTMMUUza0FDOdIpcE3e5VEu_2PzPmw0wqXGLuTwNmNURiL1UzlrabQIJ4lvmOyRN4wuiQfzw-5oyo0W1sPwct427mq22GFyguLAiHWiIXDraFdXafJBo2Ceco4UH_MyoD16TrSOcA8iZ0Dz1r7gJ_QJGIYd7BjWkkI_2fHL6msEyGk9NtkyVe89OVsvKb8SM0gxt_xT75Sr01Ktr790E-jkgMMAkxwktX4NKdFxK_6btJC8jbSUcY37ln_0fOoN-Zd9Kre2Oc1qc38s0w".to_string(),
        };

        let response = server.post("/check_sync_availability").json(&payload).await;

        response.assert_status_not_found();

        response.assert_json(&json!(
            {
                "status": "center_not_found"
            }
        ));
    }

    async fn generate_signature(center_id: String, private_key: String) -> Result<String, ()> {
        tokio::task::spawn_blocking(move || {
            let private_key_der = base64::prelude::BASE64_STANDARD
                .decode(private_key)
                .map_err(|_| ())?;

            // Load RSA private key
            let rsa_private_key = Rsa::private_key_from_der(&private_key_der)
                .map_err(|_| ())?
                .private_key_to_der()
                .map_err(|_| ())?;

            // Convert the RSA key into the correct format for jsonwebtoken
            let encoding_key = EncodingKey::from_rsa_der(&rsa_private_key);

            // Define your claims
            let my_claims = serde_json::json!({ "center_id": center_id });

            // Create the header and set the algorithm to RS256
            let header = Header::new(Algorithm::RS256);

            // Encode the token
            jsonwebtoken::encode(&header, &my_claims, &encoding_key).map_err(|_| ())
        })
        .await
        .map_err(|_| ())?
    }

    #[tokio::test]
    async fn test_unauthorized_center() {
        let (db, server) = setup().await;

        let public_key = "MIIBCgKCAQEA2X259apxTri5rV1mFJadvzc7YZZgdxuvQPoxBRTf6x2cAULCnx/UkQAwfNKxTp4pQ9thrLOwx5a8OZN74xpqQXzTjqn7OkQ8pm3qpmQ+av+XD2LLnRisMA2C//i8A3qeQc5CAyy+6gMPyMEz7ku718qlxZxAdqO1sjB0bIdaRHHXoTt2+MAv1bba6Q3aePZbj+NQY9okE/4wE3Y5iKS7C/4leXP1nhqAEnwio/sv3BgUF7bvYZhaGQ0sdBXBviDwYAixW4MtPGujZ+UWmZ4CNZdA7p18lPdSqMpgGd5oFOaTLifrQGCSCExgoqVcF5kSJ3pBpcNXGdvdZpA8CH7yXwIDAQAB";
        let private_key = "MIIEpQIBAAKCAQEA2X259apxTri5rV1mFJadvzc7YZZgdxuvQPoxBRTf6x2cAULCnx/UkQAwfNKxTp4pQ9thrLOwx5a8OZN74xpqQXzTjqn7OkQ8pm3qpmQ+av+XD2LLnRisMA2C//i8A3qeQc5CAyy+6gMPyMEz7ku718qlxZxAdqO1sjB0bIdaRHHXoTt2+MAv1bba6Q3aePZbj+NQY9okE/4wE3Y5iKS7C/4leXP1nhqAEnwio/sv3BgUF7bvYZhaGQ0sdBXBviDwYAixW4MtPGujZ+UWmZ4CNZdA7p18lPdSqMpgGd5oFOaTLifrQGCSCExgoqVcF5kSJ3pBpcNXGdvdZpA8CH7yXwIDAQABAoIBAQDD7vk1sWxMkCxuW4MYLyxD1J1BaDjVdPJopjy9KDYl2VHu7NbqhcF5M+N6wFEN03y3bg9Lh8JNvKUrdYuZZ/Wrs6nfj5ENx+WfxcwsRIja2hGbwdRPXafZzoJi2hF/TNr/y7I6q/f+V+3DXRLMrhu25xB6uy5z8z5AHlj0YOfYWdGX57oFt7AjQv57q51PKDTXas1pwidlPf8tqAZhTc+cz5y84e/l1nsDp1XL9xv+Qpy2IqtYSi7jCqI1YzcYpQZJPPWCbE43lfIMSz8CozFf0kQKfuJqPBWkij+BOx6bA08KA2tH8deOS/XoB1eOsorZUJja7PWnVP0iZdLzCeVZAoGBAPPX4xBBfuLNV3/6FAmtmnLThhtr4JFQlXh9MSfR5c2d1POWD1POG4ZvM5Bd03K/EYHg2oocguynMHq8bACMYllEmlH6BeRQJNhNOJV8cZNfC1gYMK/QKFqAh66wNDN1Ja9cATesIv62yLy0711RyEuPkFkZr+xEtoBnrQTIsT1LAoGBAORVg0jHQ8Xl6Nzo9QQJsNDrgZzJZB/vuslbeAVhhGRpNVDcvj1837Izv3Cm0UILP/mlR3G7Sv9mtkj8f8aquwSJh25mIk8g0bGEP8UZH3WuH7OfjtZu4n2M+PocpG9icFuUOQ3q3BMdvq2TfcbdUyFGySh3POxMj2OCzQtkONa9AoGAF+2EY5D5wYnC31UL6FM3x7LIAyLX24qb0EIAs9aeBUpKnkiIYoHkI5H/7le2qxjiv+rvpwPbORvC3xhkRL2B3R6lQgwVzeyrYOpa9hhLENoPw+pDxMzZWOAp8FNsG+yP8SBHIk5q9LG9Cv96SZ7/16JT2Npzb+ziN8F0sfZ7pfcCgYEAni3jUhLBD6KzS/6SZma0ODc+Rjh6BWnVE1MrdUbWKZ180vTpUb1lVpVkxQy7oK4cdryaHt7qGL61/x/1ANMb1gvUZ5WXpQuOWRTN/KPn2GV1DsG1eTW9784uWU5oV8VxIvAvCkYuiYusoaCwnIiM41ufVUotSWHMX9qoY4Ddo10CgYEAr2DMNA9UsWtaKWoU4xo8Y/e7dUVDPDIbCIqMrpJ+aScCg02S7L7bnAJcbYRnseabJA+ZmWFqX9hfEoeO0i8KU4IPUcMY/ujMqumLoGTjVseMxTJks7DzPZM9aTuNx8GdOZpyZoNED+uZB0m3/3MJ9CkOT7RcbaB6CKo6WVY2c7o=";

        let center: Vec<Record> = db
            .surreal
            .create("center")
            .content(Center {
                name: "name".to_string(),
                public_key: public_key.to_string(),
                owner: Thing::from(("user", "user_id")),
                address: Address {
                    city: "city".to_string(),
                    country: "country".to_string(),
                    line1: "line1".to_string(),
                    state: "state".to_string(),
                    landmark: None,
                },
            })
            .await
            .unwrap();

        let center = center.first().unwrap();

        let payload = CheckSyncAvailabilityPayload {
            center_id: center.id.to_string(),
            signature: generate_signature("invalid_center_id".to_string(), private_key.to_string())
                .await
                .unwrap(),
        };

        let response = server.post("/check_sync_availability").json(&payload).await;

        response.assert_status_unauthorized();

        response.assert_json(&json!(
            {
                "status": "center_signature_invalid"
            }
        ));
    }

    #[tokio::test]
    async fn test_authorized_center() {
        let (db, server) = setup().await;

        let public_key = "MIIBCgKCAQEA2X259apxTri5rV1mFJadvzc7YZZgdxuvQPoxBRTf6x2cAULCnx/UkQAwfNKxTp4pQ9thrLOwx5a8OZN74xpqQXzTjqn7OkQ8pm3qpmQ+av+XD2LLnRisMA2C//i8A3qeQc5CAyy+6gMPyMEz7ku718qlxZxAdqO1sjB0bIdaRHHXoTt2+MAv1bba6Q3aePZbj+NQY9okE/4wE3Y5iKS7C/4leXP1nhqAEnwio/sv3BgUF7bvYZhaGQ0sdBXBviDwYAixW4MtPGujZ+UWmZ4CNZdA7p18lPdSqMpgGd5oFOaTLifrQGCSCExgoqVcF5kSJ3pBpcNXGdvdZpA8CH7yXwIDAQAB";
        let private_key = "MIIEpQIBAAKCAQEA2X259apxTri5rV1mFJadvzc7YZZgdxuvQPoxBRTf6x2cAULCnx/UkQAwfNKxTp4pQ9thrLOwx5a8OZN74xpqQXzTjqn7OkQ8pm3qpmQ+av+XD2LLnRisMA2C//i8A3qeQc5CAyy+6gMPyMEz7ku718qlxZxAdqO1sjB0bIdaRHHXoTt2+MAv1bba6Q3aePZbj+NQY9okE/4wE3Y5iKS7C/4leXP1nhqAEnwio/sv3BgUF7bvYZhaGQ0sdBXBviDwYAixW4MtPGujZ+UWmZ4CNZdA7p18lPdSqMpgGd5oFOaTLifrQGCSCExgoqVcF5kSJ3pBpcNXGdvdZpA8CH7yXwIDAQABAoIBAQDD7vk1sWxMkCxuW4MYLyxD1J1BaDjVdPJopjy9KDYl2VHu7NbqhcF5M+N6wFEN03y3bg9Lh8JNvKUrdYuZZ/Wrs6nfj5ENx+WfxcwsRIja2hGbwdRPXafZzoJi2hF/TNr/y7I6q/f+V+3DXRLMrhu25xB6uy5z8z5AHlj0YOfYWdGX57oFt7AjQv57q51PKDTXas1pwidlPf8tqAZhTc+cz5y84e/l1nsDp1XL9xv+Qpy2IqtYSi7jCqI1YzcYpQZJPPWCbE43lfIMSz8CozFf0kQKfuJqPBWkij+BOx6bA08KA2tH8deOS/XoB1eOsorZUJja7PWnVP0iZdLzCeVZAoGBAPPX4xBBfuLNV3/6FAmtmnLThhtr4JFQlXh9MSfR5c2d1POWD1POG4ZvM5Bd03K/EYHg2oocguynMHq8bACMYllEmlH6BeRQJNhNOJV8cZNfC1gYMK/QKFqAh66wNDN1Ja9cATesIv62yLy0711RyEuPkFkZr+xEtoBnrQTIsT1LAoGBAORVg0jHQ8Xl6Nzo9QQJsNDrgZzJZB/vuslbeAVhhGRpNVDcvj1837Izv3Cm0UILP/mlR3G7Sv9mtkj8f8aquwSJh25mIk8g0bGEP8UZH3WuH7OfjtZu4n2M+PocpG9icFuUOQ3q3BMdvq2TfcbdUyFGySh3POxMj2OCzQtkONa9AoGAF+2EY5D5wYnC31UL6FM3x7LIAyLX24qb0EIAs9aeBUpKnkiIYoHkI5H/7le2qxjiv+rvpwPbORvC3xhkRL2B3R6lQgwVzeyrYOpa9hhLENoPw+pDxMzZWOAp8FNsG+yP8SBHIk5q9LG9Cv96SZ7/16JT2Npzb+ziN8F0sfZ7pfcCgYEAni3jUhLBD6KzS/6SZma0ODc+Rjh6BWnVE1MrdUbWKZ180vTpUb1lVpVkxQy7oK4cdryaHt7qGL61/x/1ANMb1gvUZ5WXpQuOWRTN/KPn2GV1DsG1eTW9784uWU5oV8VxIvAvCkYuiYusoaCwnIiM41ufVUotSWHMX9qoY4Ddo10CgYEAr2DMNA9UsWtaKWoU4xo8Y/e7dUVDPDIbCIqMrpJ+aScCg02S7L7bnAJcbYRnseabJA+ZmWFqX9hfEoeO0i8KU4IPUcMY/ujMqumLoGTjVseMxTJks7DzPZM9aTuNx8GdOZpyZoNED+uZB0m3/3MJ9CkOT7RcbaB6CKo6WVY2c7o=";

        let center: Vec<Record> = db
            .surreal
            .create("center")
            .content(Center {
                name: "name".to_string(),
                public_key: public_key.to_string(),
                owner: Thing::from(("user", "user_id")),
                address: Address {
                    city: "city".to_string(),
                    country: "country".to_string(),
                    line1: "line1".to_string(),
                    state: "state".to_string(),
                    landmark: None,
                },
            })
            .await
            .unwrap();

        let center = center.first().unwrap();

        let payload = CheckSyncAvailabilityPayload {
            center_id: center.id.to_string(),
            signature: generate_signature(center.id.to_string(), private_key.to_string())
                .await
                .unwrap(),
        };

        let response = server.post("/check_sync_availability").json(&payload).await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": "available"
            }
        ));
    }
}
