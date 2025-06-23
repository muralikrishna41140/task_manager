use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description : Option<String>,
}

