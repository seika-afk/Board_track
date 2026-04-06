use crate::route::create_router;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

mod handlers;
mod model;
mod route;
mod schema;

//########### STRUCTS #########//
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    println!("Connected to DB");

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started at localhost:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
