use serde::{Deserialize, Serialize};
// FromRow allows deserialization of query results into Rust structs
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct TodolistEntry {
    id: i32,
    complete: bool,
    title: String,
}

#[derive(Deserialize)]
pub struct CreateEntryBody {
    pub title: String,
}

#[derive(Deserialize)]
pub struct CompleteEntryBody {
    pub complete: bool,
}
