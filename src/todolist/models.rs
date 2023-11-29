use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}

#[derive(Deserialize)]
pub struct CreateEntryBody {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize)]
pub struct UpdateEntryBody {
    pub title: String,
}
