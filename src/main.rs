mod database;
mod models;
mod handlers;

use axum::{
    routing::get,
    Router,
    response::Json
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use axum::response::IntoResponse;

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "message": "Deck cards API is running!"
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables
    dotenvy::dotenv().ok();

    // Test database connection
    let _db = database::establish_connection().await?;
    println!("✅ Database connected successfully!");

    // Create router
    let app: Router<()> = Router::new().route("/health", get(health_check));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
