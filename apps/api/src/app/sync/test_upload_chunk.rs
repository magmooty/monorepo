#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum_test::TestServer;
    use base64::Engine;
    use bytes::Bytes;
    use rsa::pkcs1v15::SigningKey;
    use rsa::signature::SignatureEncoding;
    use rsa::{pkcs1::DecodeRsaPrivateKey, sha2::Sha256, signature::SignerMut, RsaPrivateKey};
    use serde_json::json;
    use surrealdb::sql::{Datetime, Thing};
    use telegram_bot::TelegramClient;

    use crate::app::sync::UploadChunkPayload;
    use crate::database::{
        local_structs::{Content, User},
        SyncEvent,
    };
    use crate::{
        app::{sync::get_router, AppState},
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

    async fn generate_signature(
        center_id: &String,
        body: Bytes,
        private_key: String,
    ) -> Result<String, ()> {
        let center_id = center_id.clone();

        tokio::task::spawn_blocking(move || {
            let private_key_der = base64::prelude::BASE64_STANDARD
                .decode(private_key)
                .unwrap();

            let private_key = RsaPrivateKey::from_pkcs1_der(&private_key_der).unwrap();

            let mut signing_key = SigningKey::<Sha256>::new(private_key);

            let mut bytes_to_sign = center_id.as_bytes().to_vec();

            bytes_to_sign.extend(body);

            let signature = signing_key.sign(&bytes_to_sign);

            Ok(base64::prelude::BASE64_STANDARD.encode(signature.to_bytes()))
        })
        .await
        .map_err(|_| ())?
    }

    #[tokio::test]
    async fn test_authorized_sync() {
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

        let payload: UploadChunkPayload = UploadChunkPayload {
            chunk: vec![SyncEvent {
                record_id: Thing::from(("user", "id1")),
                event: "CREATE".to_string(),
                content: Content::User(User {
                    name: "name".to_string(),
                    phone_number: "phone_number".to_string(),
                    password: "password".to_string(),
                }),
                created_at: Datetime::default(),
            }],
        };

        let payload = serde_json::to_string(&payload).unwrap();

        let signature = generate_signature(
            &center.id.to_string(),
            payload.bytes().collect(),
            private_key.to_string(),
        )
        .await
        .unwrap();

        let response = server
            .post("/upload_chunk")
            .add_header("Signature".parse().unwrap(), signature.parse().unwrap())
            .add_header(
                "Content-Type".parse().unwrap(),
                "application/json".parse().unwrap(),
            )
            .add_header(
                "Center-ID".parse().unwrap(),
                center.id.to_string().parse().unwrap(),
            )
            .bytes(payload.bytes().collect())
            .await;

        response.assert_status_success();

        response.assert_json(&json!(
            {
                "status": "accepted"
            }
        ));
    }

    #[tokio::test]
    async fn test_unauthorized_sync() {
        let (db, server) = setup().await;

        let public_key = "MIIBCgKCAQEA2X259apxTri5rV1mFJadvzc7YZZgdxuvQPoxBRTf6x2cAULCnx/UkQAwfNKxTp4pQ9thrLOwx5a8OZN74xpqQXzTjqn7OkQ8pm3qpmQ+av+XD2LLnRisMA2C//i8A3qeQc5CAyy+6gMPyMEz7ku718qlxZxAdqO1sjB0bIdaRHHXoTt2+MAv1bba6Q3aePZbj+NQY9okE/4wE3Y5iKS7C/4leXP1nhqAEnwio/sv3BgUF7bvYZhaGQ0sdBXBviDwYAixW4MtPGujZ+UWmZ4CNZdA7p18lPdSqMpgGd5oFOaTLifrQGCSCExgoqVcF5kSJ3pBpcNXGdvdZpA8CH7yXwIDAQAB";

        // This is a private key of another pair
        let private_key = "MIIEowIBAAKCAQEAxWQfNO4t+YvZw2gTdIgFV1bDBCMAd3pFPiF+agog898jsMJEI1QaeY0h584cM9wT9OB6UIE5cupiC297Q+draA8c7HCaF6JH93OxI7Frw0dzWbeHnYiO7M+KNj1/gDLpKBlwokP4JA3ao0A55qM9JemLeZCT4EzhQNc0BLkXaBhHVS0o2kKuU3lVZraBsBukN4gy99Fsoya3RShWXjnTJZUgzfW16yMWvIho3DswMhYZYZoRadwb7wFCigcgvYtSIh7hpSoujuhI9uBWgFK6EguDfxZvi/uJExzLmrdHmhRDAzj06f3Sn0FIkdS9vwUKohE6sFOvbcgq+Jp2lWMH1wIDAQABAoIBADjaSZK5N0y01xfNFi1uL/uj5a9/VVcURHVMuFhaantTPbhfe7ihNK7l02oslzronGbfcNtXHPDYSTz58wrv0KO8FpyTIg7Eku3WyKJ7K4qnPbYNgmYBOwlCDZDpzZtjDnyaNzLFWOcphluW2BZx1d7hkWaGGdF00zwVv8nMdc1pDy0C3pxb2HEbxXhcqJtZQP53VVupmR4mLDJdocl3FHFpve1AxEQ9Tar2z5zfMyt7dkDCShz2RSTj3y3oPff2ZHldaIeXzsYTAw1QI+otwMuCfaiMJN/1W72JALO4hK1bYH5wMxWXdFn54epg0gDgEslPLEgeUcIXCgLFAfJ25EECgYEA2HjpKNnJb+racuSxxtfwPIS6S3IVJowS3uO+rUF7y7kJALtgzHNTkfuB4VkQSXmx3R3nWKh4sNdu5ie4AsVlJsCCmc2DhMZym/0aSwN3r4+HdMzy7EwKPSOfK6T7xorzU+CoEA7lK2N6VKCHPwHaEGK9mHz42MwD0GJxYpOIMYkCgYEA6W9AiICTb2HkEj64mw31iAjWpAzwBDxNQ0wtObKQ0FrVJzFbxHTqBTYkYq6HSU7lbrZ7L63MoeUNZWtgXq9zae71D2j6m7UDrwhgoyLHsCsheLYFKxgX+ucgV0G36xcMWUZ4IsieBk/tarCL/AfXHQ5sBiLvudH7vZDpYV+H9l8CgYEAlrjBJwdUoQ81oG+pdFif1ZNYnvVE0r0O2pBaEAobJwKjyRWuVUwrMOacTQoVNAN+mwWaMfnDZuXpOmkHTy6fujap+GrGmukNnZzB9N7qM5vM5b9ZmLttGZq21c2e3GOc/pbajavBVs+BunC3GmCws138wrKpFVIfibUxW0B7wHkCgYAwgW+VbbI1KNAtgl71yTbF56BCQnX1S1p6q2+SV6qDwPwdwsHg1rJyFsgEwWNXNk+ya7S5OZIV6fSYqHFD+40t6/t8EkJQ7JIxYrpB6842qx+vZ5M3WzBQcIpl10ASV523R09bWviLMzyQONM9sZtfbEnCFKxfnmSDBYOZOjKUXQKBgHAM4fww9LW32QR6402AFeGqstuAHCpAnjBZ2Yo/ZLX9B0EzonmJP4NpVIQD0xc2vUlUW/G8SAQttxc6XV1WKi0ixp+0J7ZiRlQ+I6L5OtFAKB2ylb22t9zMECAHTklXwl9IFw14VcdplXi1MF0yVO02SMyzcdd3VzVgX4SU6uBj";

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

        let payload: UploadChunkPayload = UploadChunkPayload {
            chunk: vec![SyncEvent {
                record_id: Thing::from(("user", "id1")),
                event: "CREATE".to_string(),
                content: Content::User(User {
                    name: "name".to_string(),
                    phone_number: "phone_number".to_string(),
                    password: "password".to_string(),
                }),
                created_at: Datetime::default(),
            }],
        };

        let payload = serde_json::to_string(&payload).unwrap();

        let response = server
            .post("/upload_chunk")
            .add_header(
                "Content-Type".parse().unwrap(),
                "application/json".parse().unwrap(),
            )
            .add_header(
                "Center-ID".parse().unwrap(),
                center.id.to_string().parse().unwrap(),
            )
            .bytes(payload.bytes().collect())
            .await;

        response.assert_status_bad_request();

        response.assert_json(&json!(
            {
                "status": "missing_headers"
            }
        ));

        let signature = generate_signature(
            &center.id.to_string(),
            payload.bytes().collect(),
            private_key.to_string(),
        )
        .await
        .unwrap();

        let response = server
            .post("/upload_chunk")
            .add_header("Signature".parse().unwrap(), signature.parse().unwrap())
            .add_header(
                "Content-Type".parse().unwrap(),
                "application/json".parse().unwrap(),
            )
            .add_header(
                "Center-ID".parse().unwrap(),
                "center:no_center_here".to_string().parse().unwrap(),
            )
            .bytes(payload.bytes().collect())
            .await;

        response.assert_status_not_found();

        response.assert_json(&json!(
            {
                "status": "center_not_found"
            }
        ));

        let response = server
            .post("/upload_chunk")
            .add_header("Signature".parse().unwrap(), signature.parse().unwrap())
            .add_header(
                "Content-Type".parse().unwrap(),
                "application/json".parse().unwrap(),
            )
            .add_header(
                "Center-ID".parse().unwrap(),
                center.id.to_string().parse().unwrap(),
            )
            .bytes(payload.bytes().collect())
            .await;

        response.assert_status_unauthorized();

        response.assert_json(&json!(
            {
                "status": "signature_invalid"
            }
        ));
    }
}
