use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Eq)]
#[sea_orm(table_name = "cards")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub deck_id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::deck::Entity",
    from = "Column::DeckId",
    to = "super::deck::Column::Id",)]
    Deck
}

impl Related<super::deck::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Deck.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
