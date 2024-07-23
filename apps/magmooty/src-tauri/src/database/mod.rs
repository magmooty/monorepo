use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

pub async fn create_database() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<RocksDb>("rocksdb").await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}
