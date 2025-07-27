mod database;
mod handlers;
mod migration;
mod models;

use axum::{
    Router,
    response::Json,
    routing::{delete, get, post, put},
};
use handlers::deck_handlers;
use sea_orm_migration::prelude::*;
use serde_json::{Value, json};
use std::net::SocketAddr;

use crate::handlers::card_handlers;

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
    let app: Router<()> = Router::new()
        .route("/health", get(health_check))
        // Deck routes
        .route("/decks", post(deck_handlers::create_deck))
        .route("/decks", get(deck_handlers::get_all_decks))
        .route("/decks/:id", get(deck_handlers::get_deck_by_id))
        .route("/decks/:id", put(deck_handlers::update_deck_by_id))
        .route("/decks/:id", delete(deck_handlers::delete_deck_by_id))
        // Card routes
        .route("/cards", post(card_handlers::create_card))
        .route("/cards", get(card_handlers::get_all_cards))
        .route("/cards/:id", get(card_handlers::get_card_by_id))
        .route("/cards/:id", put(card_handlers::update_card))
        .route("/cards/:id", delete(card_handlers::delete_card))
        .route("/decks/:id/cards", get(card_handlers::get_cards_by_deck))
        .with_state(db);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
