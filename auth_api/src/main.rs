use axum::{Router, routing::get};
use handler::check_health;
use std::net::TcpListener;

mod config;
mod db;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app: Router<()> = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/health", get(check_health()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(());

    let db = db::connect_db().await?;
    handler::get_user(db.clone()).await?
}
