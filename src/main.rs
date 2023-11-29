use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod todolist;
use todolist::services;

// information shared with each route
struct AppState {
    db: Pool<Postgres>,
}

#[get("/")] // GET method for the "/" path
async fn index() -> impl Responder {
    HttpResponse::Ok().json("{ status: OK }")
}

// This tells our program to utilize the actix_web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // configure our API to utilize the .env file
    dotenv().ok();

    // retrieve the database url from the .env file
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    // create a connection pool to the database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    HttpServer::new(move || {
        App::new()
            // share the pool with each route
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(index)
            .configure(services::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
