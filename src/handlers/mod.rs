pub mod card_handlers;
pub mod deck_handlers;

use serde::{Deserialize, Serialize};

// Decks
#[derive(Deserialize)]
pub struct CreateDeckRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateDeckRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct DeckResponse {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Convert from our model to response DTO
impl From<crate::models::deck::Model> for DeckResponse {
    fn from(deck: crate::models::deck::Model) -> Self {
        Self {
            id: deck.id,
            name: deck.name,
            description: deck.description,
            created_at: deck.created_at,
            updated_at: deck.updated_at,
        }
    }
}

// Cards
#[derive(Deserialize)]
pub struct CreateCardRequest {
    pub question: String,
    pub answer: String,
    pub deck_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateCardRequest {
    pub question: Option<String>,
    pub answer: Option<String>,
    pub deck_id: Option<i32>,
}

#[derive(Serialize)]
pub struct CardResponse {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub deck_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<crate::models::card::Model> for CardResponse {
    fn from(card: crate::models::card::Model) -> Self {
        Self {
            id: card.id,
            question: card.question,
            answer: card.answer,
            deck_id: card.deck_id,
            created_at: card.created_at,
            updated_at: card.updated_at,
        }
    }
}
