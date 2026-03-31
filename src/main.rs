use axum::{Json,Router,response::IntoResponse,routing::get };
use serde_json::json;


mod handlers;
use handlers::hello_world;


#[tokio::main]
async fn main()->Result<(),Box<dyn std::error::Error>>{

        let app = Router::new().route("/api", get(hello_world));
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        println!("Server Started : 0.0.0.0:3000 ");

        axum::serve(listener, app).await?;


Ok(())
}



