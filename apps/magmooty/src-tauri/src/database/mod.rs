use surrealdb::engine::local::{Db, Mem};
use surrealdb::Surreal;

pub async fn create_database() -> surrealdb::Result<Surreal<Db>> {
    let db = Surreal::new::<Mem>(()).await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}
