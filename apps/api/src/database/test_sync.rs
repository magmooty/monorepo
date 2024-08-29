#[cfg(test)]
mod tests {
    use surrealdb::sql::{Datetime, Thing};

    use crate::database::{
        local_structs::{Content, User},
        Database, SyncEvent,
    };

    async fn setup() -> Database {
        Database::in_memory().await
    }

    #[tokio::test]
    async fn test_crud_sync_events() {
        let db = setup().await;

        let center_id = "center:center1".to_string();

        // Test Insert
        let events = vec![SyncEvent {
            record_id: Thing::from(("user", "id1")),
            event: "CREATE".to_string(),
            content: Content::User(User {
                name: "name".to_string(),
                phone_number: "phone_number".to_string(),
                password: "password".to_string(),
            }),
            created_at: Datetime::default(),
        }];

        let result = db.sync.insert_sync_events(&center_id, events).await;

        assert_eq!(result.is_ok(), true);

        db.surreal
            .use_ns("magmooty")
            .use_db("center1")
            .await
            .unwrap();

        let stored_users: Vec<User> = db.surreal.select("user").await.unwrap();

        assert_eq!(stored_users.len(), 1);
        assert_eq!(stored_users[0].name, "name");
        assert_eq!(stored_users[0].phone_number, "phone_number");
        assert_eq!(stored_users[0].password, "password");

        // Test Update
        let events = vec![SyncEvent {
            record_id: Thing::from(("user", "id1")),
            event: "UPDATE".to_string(),
            content: Content::User(User {
                name: "ziad".to_string(),
                phone_number: "phone_number".to_string(),
                password: "password".to_string(),
            }),
            created_at: Datetime::default(),
        }];

        let result = db.sync.insert_sync_events(&center_id, events).await;

        assert_eq!(result.is_ok(), true);

        db.surreal
            .use_ns("magmooty")
            .use_db("center1")
            .await
            .unwrap();

        let stored_users: Vec<User> = db.surreal.select("user").await.unwrap();

        assert_eq!(stored_users.len(), 1);
        assert_eq!(stored_users[0].name, "ziad");
        assert_eq!(stored_users[0].phone_number, "phone_number");
        assert_eq!(stored_users[0].password, "password");

        // Test Delete
        let events = vec![SyncEvent {
            record_id: Thing::from(("user", "id1")),
            event: "DELETE".to_string(),
            content: Content::User(User {
                name: "ziad".to_string(),
                phone_number: "phone_number".to_string(),
                password: "password".to_string(),
            }),
            created_at: Datetime::default(),
        }];

        let result = db.sync.insert_sync_events(&center_id, events).await;

        assert_eq!(result.is_ok(), true);

        db.surreal
            .use_ns("magmooty")
            .use_db("center1")
            .await
            .unwrap();

        let stored_users: Vec<User> = db.surreal.select("user").await.unwrap();

        assert_eq!(stored_users.len(), 0);
    }

    #[tokio::test]
    async fn test_create_overwrite_sync_events() {
        let db = setup().await;

        let center_id = "center:center1".to_string();

        let events = vec![
            SyncEvent {
                record_id: Thing::from(("user", "id1")),
                event: "CREATE".to_string(),
                content: Content::User(User {
                    name: "name".to_string(),
                    phone_number: "phone_number".to_string(),
                    password: "password".to_string(),
                }),
                created_at: Datetime::default(),
            },
            SyncEvent {
                record_id: Thing::from(("user", "id1")),
                event: "CREATE".to_string(),
                content: Content::User(User {
                    name: "ziad".to_string(),
                    phone_number: "phone_number".to_string(),
                    password: "password".to_string(),
                }),
                created_at: Datetime::default(),
            },
        ];

        let result = db.sync.insert_sync_events(&center_id, events).await;

        assert_eq!(result.is_ok(), true);

        db.surreal
            .use_ns("magmooty")
            .use_db("center1")
            .await
            .unwrap();

        let stored_users: Vec<User> = db.surreal.select("user").await.unwrap();

        assert_eq!(stored_users.len(), 1);
        assert_eq!(stored_users[0].name, "ziad");
        assert_eq!(stored_users[0].phone_number, "phone_number");
        assert_eq!(stored_users[0].password, "password");
    }

    #[tokio::test]
    async fn test_upsert_sync_events() {
        let db = setup().await;

        let center_id = "center:center1".to_string();

        let events = vec![SyncEvent {
            record_id: Thing::from(("user", "id1")),
            event: "UPDATE".to_string(),
            content: Content::User(User {
                name: "name".to_string(),
                phone_number: "phone_number".to_string(),
                password: "password".to_string(),
            }),
            created_at: Datetime::default(),
        }];

        let result = db.sync.insert_sync_events(&center_id, events).await;

        assert_eq!(result.is_ok(), true);

        db.surreal
            .use_ns("magmooty")
            .use_db("center1")
            .await
            .unwrap();

        let stored_users: Vec<User> = db.surreal.select("user").await.unwrap();

        assert_eq!(stored_users.len(), 1);
        assert_eq!(stored_users[0].name, "name");
        assert_eq!(stored_users[0].phone_number, "phone_number");
        assert_eq!(stored_users[0].password, "password");
    }
}
