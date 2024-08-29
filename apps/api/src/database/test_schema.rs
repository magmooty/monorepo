#[cfg(test)]
mod tests {

    use crate::database::{schema::LOCAL_SCHEMA, Database};

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_crud_sync_events() {
        let db = setup().await;

        let result = db.surreal.query(LOCAL_SCHEMA).await;

        assert_eq!(result.is_ok(), true);
    }
}
