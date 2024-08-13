#[cfg(test)]
mod tests {
    use crate::database::Record;

    use crate::database::{
        center::{Address, Center},
        Database,
    };

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_find_center() {
        let db = setup().await;

        let user = db.user.create_user(&"+201096707442".to_string()).await;

        let centers: Vec<Record> = db
            .surreal
            .create("center")
            .content(Center {
                name: "Test".to_string(),
                public_key: "public_key".to_string(),
                address: Address {
                    line1: "line1".to_string(),
                    landmark: None,
                    city: "city".to_string(),
                    state: "state".to_string(),
                    country: "country".to_string(),
                },
                owner: user.id,
            })
            .await
            .unwrap();

        let center = centers.first().take().unwrap();

        let center = db.center.get_center(&center.id.to_string()).await;

        assert!(center.is_some());

        let center = center.unwrap();

        assert_eq!(center.name, "Test");
        assert_eq!(center.public_key, "public_key");
    }
}
