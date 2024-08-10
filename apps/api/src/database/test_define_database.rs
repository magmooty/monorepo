#[cfg(test)]
mod tests {
    use crate::database::Database;

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_define_database() {
        let db = setup().await;
        db.define_database().await;
    }
}
