mod database;
mod models;
mod handlers;
mod migration;

use axum::{
    routing::get,
    Router,
    response::Json
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use sea_orm_migration::prelude::*;

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
    let db = database::establish_connection().await?;
    println!("✅ Database connected successfully!");

    // Run migrations
    let schema_manager = SchemaManager::new(&db);
    migration::Migration.up(&schema_manager).await?;
    println!("✅ Migrations completed successfully!");

    // Create router
    let app: Router<()> = Router::new().route("/health", get(health_check));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
