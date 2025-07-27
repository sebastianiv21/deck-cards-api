use super::{CreateDeckRequest, DeckResponse};
use crate::{
    handlers::UpdateDeckRequest,
    models::{Deck, deck},
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn create_deck(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateDeckRequest>,
) -> Result<ResponseJson<DeckResponse>, StatusCode> {
    let new_deck = deck::ActiveModel {
        name: Set(payload.name),
        description: Set(payload.description),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    match new_deck.insert(&db).await {
        Ok(deck) => Ok(ResponseJson(DeckResponse::from(deck))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all_decks(
    State(db): State<DatabaseConnection>,
) -> Result<ResponseJson<Vec<DeckResponse>>, StatusCode> {
    match Deck::find().all(&db).await {
        Ok(decks) => {
            let deck_responses: Vec<DeckResponse> =
                decks.into_iter().map(DeckResponse::from).collect();
            Ok(ResponseJson(deck_responses))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_deck_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ResponseJson<DeckResponse>, StatusCode> {
    match Deck::find_by_id(id).one(&db).await {
        Ok(Some(deck)) => Ok(ResponseJson(DeckResponse::from(deck))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_deck_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateDeckRequest>,
) -> Result<ResponseJson<DeckResponse>, StatusCode> {
    // Find the existing deck
    let existing_deck = match Deck::find_by_id(id).one(&db).await {
        Ok(Some(deck)) => deck,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create ActiveModel for update
    let mut deck_update: deck::ActiveModel = existing_deck.into();

    // Only update fields that were provided
    if let Some(name) = payload.name {
        deck_update.name = Set(name);
    }
    if payload.description.is_some() {
        deck_update.description = Set(payload.description);
    }
    deck_update.updated_at = Set(chrono::Utc::now());

    match deck_update.update(&db).await {
        Ok(updated_deck) => Ok(ResponseJson(DeckResponse::from(updated_deck))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_deck_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match Deck::delete_by_id(id).exec(&db).await {
        Ok(delete_result) => {
            if delete_result.rows_affected == 1 {
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
