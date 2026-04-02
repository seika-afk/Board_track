use axum::{Json, Router, response::IntoResponse, routing::get};
use dotenv::dotenv;
use serde_json::json;

mod handlers;
use handlers::hello_world;
use sqlx::{PgPool, postgres::PgPoolOptions};

//########### STRUCTS #########//
pub struct AppState {
    db: PgPool,
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    println!("Connected to DB");

    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server Started : 0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
