use crate::AppState;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tauri::command]
#[specta::specta]
pub async fn create_student<'a>(state: State<'a, AppState>) -> Result<(), ()> {
    let student: Vec<Record> = state
        .database
        .create("student")
        .content(Student {
            name: "John Doe".to_string(),
        })
        .await
        .unwrap();

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_students<'a>(state: State<'a, AppState>) -> Result<(), ()> {
    let result: Vec<Student> = state.database.select("student").await.unwrap();

    Ok(())
}
