use axum::{Router, routing::get};
use std::net::TcpListener;

mod config;
mod db;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router<()> = Router::new().route("/", get(hello_world()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn hello_world() -> &'static str {
    return "Hello World";
}
