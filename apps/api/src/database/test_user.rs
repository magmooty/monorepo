#[cfg(test)]
mod tests {
    use crate::database::Database;

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_create_user() {
        let db = setup().await;

        db.user.create_user(&"+201096707442".to_string()).await;

        let user = db.user.find_user(&"+201096707442".to_string()).await;

        assert!(user.is_some());

        let user = user.unwrap();

        assert_eq!(user.phone_number, "+201096707442");
    }
}
