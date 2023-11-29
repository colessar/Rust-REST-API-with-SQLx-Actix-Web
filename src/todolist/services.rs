use super::models::{CompleteEntryBody, CreateEntryBody, TodolistEntry};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx;

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, TodolistEntry>("SELECT id, title, complete FROM todolist_entries")
        // return all matching rows
        .fetch_all(&data.db)
        .await
    {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().json("Error trying to get entries"),
    }
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    body: web::Json<CreateEntryBody>,
) -> impl Responder {
    let param_obj = body.into_inner();

    match sqlx::query_as::<_, TodolistEntry>(
        "INSERT INTO todolist_entries (title) VALUES ($1) RETURNING id, title, complete",
    )
    // bind the param_obj.title value to the $1 argument
    .bind(&param_obj.title)
    // return one row
    .fetch_one(&data.db)
    .await
    {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().json("Error trying to create entry"),
    }
}

#[put("/todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<CompleteEntryBody>,
) -> impl Responder {
    let id = path.into_inner();
    let param_obj = body.into_inner();

    match sqlx::query_as::<_, TodolistEntry>(
        "UPDATE todolist_entries SET complete=$1 WHERE id=$2 RETURNING id, title, complete",
    )
    // bind the param_obj.complete value to the $1 argument
    .bind(&param_obj.complete)
    // bind the id value to the $2 argument
    .bind(&id)
    // return the updated row
    .fetch_one(&data.db)
    .await
    {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().json("Error trying to update entry"),
    }
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match sqlx::query!(
        "DELETE FROM todolist_entries WHERE id=$1",
        // bind the id value to the $1 argument
        id
    )
    // execute the query without returning any information
    .execute(&data.db)
    .await
    {
        Ok(_) => HttpResponse::Ok().json("Successfully deleted entry"),
        Err(_) => HttpResponse::InternalServerError().json("Error trying to delete entry"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}
