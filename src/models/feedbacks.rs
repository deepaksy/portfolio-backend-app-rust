use serde::{Deserialize, Serialize};
use sqlx::PgPool;

// 1. DATABASE MODELS & DTOs
#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Feedback {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub message: String,
}


#[derive(Deserialize)]
pub struct CreateFeedbackInput {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdateFeedbackInput {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub message: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
