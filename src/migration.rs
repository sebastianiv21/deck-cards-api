use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create decks table
        manager
            .create_table(
                Table::create()
                    .table(Deck::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deck::Id)
                            .not_null()
                            .primary_key()
                            .integer()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Deck::Name).string().not_null())
                    .col(ColumnDef::new(Deck::Description).string())
                    .col(
                        ColumnDef::new(Deck::CreatedAt)
                            .not_null()
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Deck::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create cards table
        manager.create_table(
            Table::create()
                .table(Card::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Card::Id)
                        .not_null()
                        .integer()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(Card::Question).string().not_null())
                .col(ColumnDef::new(Card::Answer).string().not_null())
                .col(ColumnDef::new(Card::DeckId).integer().not_null())
                .col(
                    ColumnDef::new(Card::CreatedAt)
                        .not_null()
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp()),
                )
                .col(
                    ColumnDef::new(Card::UpdatedAt)
                        .not_null()
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp()),
                )
            .foreign_key(
                ForeignKey::create()
                    .name("fk-card-deck_id")
                    .from(Card::Table, Card::DeckId)
                    .to(Deck::Table, Deck::Id)
                    .on_delete(ForeignKeyAction::Cascade)
            )
            .to_owned(),
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Card::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Deck::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Deck {
    #[iden = "decks"]
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Card {
    #[iden = "cards"]
    Table,
    Id,
    Question,
    Answer,
    DeckId,
    CreatedAt,
    UpdatedAt,
}