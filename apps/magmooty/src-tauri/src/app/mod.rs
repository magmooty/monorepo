use serde;
use serde::Serialize;
use specta::Type;

pub mod students;
pub mod whatsapp;

#[derive(Serialize, Type)]
pub enum AppError {
    #[serde(rename = "internal_error")]
    InternalError(String),
}
