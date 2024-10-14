#[cfg(test)]
mod tests {
    use serde_json::json;
    use surrealdb::{engine::any::Any, Surreal};

    use crate::sync::{Record, Syncer};

    async fn setup() -> Surreal<Any> {
        let surreal: Surreal<Any> = Surreal::init();

        surreal.connect("mem://").await.unwrap();
        surreal.use_ns("test").use_db("test").await.unwrap();

        surreal
    }

    #[tokio::test]
    async fn test_count_sync_events() {
        let db = setup().await;

        db.create::<Vec<Record>>("sync")
            .content(json!({ "record_id": 1, "pushed": false }))
            .await
            .unwrap();

        let count = Syncer::count_sync_events(&db).await.unwrap();

        assert_eq!(count, 1);

        db.create::<Vec<Record>>("sync")
            .content(json!({ "record_id": 2, "pushed": false }))
            .await
            .unwrap();

        let count = Syncer::count_sync_events(&db).await.unwrap();

        assert_eq!(count, 2);

        for record_id in 3..351 {
            db.create::<Vec<Record>>("sync")
                .content(json!({ "record_id": record_id, "pushed": false }))
                .await
                .unwrap();
        }

        let count = Syncer::count_sync_events(&db).await.unwrap();

        assert_eq!(count, 350);

        db.query("UPDATE sync SET pushed = true WHERE record_id <= 100")
            .await
            .unwrap();

        let count = Syncer::count_sync_events(&db).await.unwrap();

        assert_eq!(count, 250);
    }

    #[tokio::test]
    async fn test_fetch_sync_events() {

    }
}
