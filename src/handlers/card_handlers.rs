use super::{CardResponse, CreateCardRequest, UpdateCardRequest};
use crate::models::{Card, Deck, card};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::Json as ResponseJson,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn create_card(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateCardRequest>,
) -> Result<ResponseJson<CardResponse>, StatusCode> {
    // Verify deck exists
    match Deck::find_by_id(payload.deck_id).one(&db).await {
        Ok(Some(_)) => {}                                // deck exists, continue
        Ok(None) => return Err(StatusCode::BAD_REQUEST), // deck doesn't exist
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    let new_card = card::ActiveModel {
        question: Set(payload.question),
        answer: Set(payload.answer),
        deck_id: Set(payload.deck_id),
        created_at: Set(chrono::Utc::now()),
        updated_at: Set(chrono::Utc::now()),
        ..Default::default()
    };

    match new_card.insert(&db).await {
        Ok(card) => Ok(ResponseJson(CardResponse::from(card))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all_cards(
    State(db): State<DatabaseConnection>,
) -> Result<ResponseJson<Vec<CardResponse>>, StatusCode> {
    match Card::find().all(&db).await {
        Ok(cards) => {
            let card_responses: Vec<CardResponse> =
                cards.into_iter().map(CardResponse::from).collect();
            Ok(ResponseJson(card_responses))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_cards_by_deck(
    State(db): State<DatabaseConnection>,
    Path(deck_id): Path<i32>,
) -> Result<ResponseJson<Vec<CardResponse>>, StatusCode> {
    use crate::models::card::Column;
    use sea_orm::ColumnTrait;

    // verify if deck exists
    match Deck::find_by_id(deck_id).one(&db).await {
        Ok(Some(_)) => {}
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    // get all cards for this deck
    match Card::find()
        .filter(Column::DeckId.eq(deck_id))
        .all(&db)
        .await
    {
        Ok(cards) => {
            let card_responses: Vec<CardResponse> =
                cards.into_iter().map(CardResponse::from).collect();
            Ok(ResponseJson(card_responses))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_card_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<ResponseJson<CardResponse>, StatusCode> {
    match Card::find_by_id(id).one(&db).await {
        Ok(Some(card)) => Ok(ResponseJson(CardResponse::from(card))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_card(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCardRequest>,
) -> Result<ResponseJson<CardResponse>, StatusCode> {
    // Find existing card
    let existing_card = match Card::find_by_id(id).one(&db).await {
        Ok(Some(card)) => card,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // If deck_id is being updated, verify the new deck exists
    if let Some(new_deck_id) = payload.deck_id {
        match Deck::find_by_id(new_deck_id).one(&db).await {
            Ok(Some(_)) => {} // New deck exists
            Ok(None) => return Err(StatusCode::BAD_REQUEST),
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    // Create ActiveModel for update
    let mut card_update: card::ActiveModel = existing_card.into();

    // Update only provided fields
    if let Some(question) = payload.question {
        card_update.question = Set(question);
    }
    if let Some(answer) = payload.answer {
        card_update.answer = Set(answer);
    }
    if let Some(deck_id) = payload.deck_id {
        card_update.deck_id = Set(deck_id);
    }
    card_update.updated_at = Set(chrono::Utc::now());

    match card_update.update(&db).await {
        Ok(updated_card) => Ok(ResponseJson(CardResponse::from(updated_card))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_card(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    match Card::delete_by_id(id).exec(&db).await {
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
