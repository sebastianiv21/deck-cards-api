# Deck Cards API

A RESTful API for managing decks of cards, built with Rust using the Axum web framework and SeaORM for PostgreSQL database interaction.

## Features

- Create, read, update, and delete (CRUD) operations for decks and cards
- Organize cards within decks
- PostgreSQL database support with migrations
- Written in async Rust for high performance
- Docker Compose for easy local development

## Tech Stack

- **Rust** (2021/2024 Edition)
- **Axum** (Web framework)
- **SeaORM** (Async ORM for Rust)
- **PostgreSQL** (Database)
- **Docker Compose** (Development environment)
- **Serde** (Serialization)
- **Tokio** (Async runtime)

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/)
- [Docker](https://www.docker.com/get-started) (for local database)
- [cargo-watch](https://crates.io/crates/cargo-watch) (optional, for live reload)

### Clone the Repository

```sh
git clone https://github.com/your-username/deck-cards-api.git
cd deck-cards-api
```

### Environment Variables

Create a `.env` file in the project root with the following content:

```
DATABASE_URL=postgres://username:password@localhost:5432/deck_cards_db
```

Adjust the credentials as needed.

### Start the Database

```sh
docker-compose up -d
```

### Run Database Migrations

Migrations are run automatically on server startup.

### Run the API Server

```sh
cargo run
```

The server will start at `http://127.0.0.1:3000`.

### Test the API

A test script is provided:

```sh
./test_api.sh
```

## API Endpoints

### Health Check

- `GET /health`
  Returns API status.

### Decks

- `POST /decks`
  Create a new deck.
  **Body:**
  ```json
  {
    "name": "Deck Name",
    "description": "Optional description"
  }
  ```

- `GET /decks`
  List all decks.

- `GET /decks/:id`
  Get a deck by ID.

- `PUT /decks/:id`
  Update a deck.
  **Body:**
  ```json
  {
    "name": "New Name",
    "description": "New description"
  }
  ```

- `DELETE /decks/:id`
  Delete a deck by ID.

### Cards

- `POST /cards`
  Create a new card.
  **Body:**
  ```json
  {
    "question": "What is ...?",
    "answer": "Answer",
    "deck_id": 1
  }
  ```

- `GET /cards`
  List all cards.

- `GET /cards/:id`
  Get a card by ID.

- `PUT /cards/:id`
  Update a card.
  **Body:**
  ```json
  {
    "question": "Updated question?",
    "answer": "Updated answer",
    "deck_id": 2
  }
  ```

- `DELETE /cards/:id`
  Delete a card by ID.

- `GET /decks/:id/cards`
  List all cards in a specific deck.

## Database Schema

### Decks

| Field       | Type    | Description         |
|-------------|---------|---------------------|
| id          | int     | Primary key         |
| name        | string  | Deck name           |
| description | string  | Optional            |
| created_at  | datetime| Creation timestamp  |
| updated_at  | datetime| Last update         |

### Cards

| Field      | Type    | Description         |
|------------|---------|---------------------|
| id         | int     | Primary key         |
| question   | string  | Card question       |
| answer     | string  | Card answer         |
| deck_id    | int     | Foreign key to deck |
| created_at | datetime| Creation timestamp  |
| updated_at | datetime| Last update         |

## Development

- Code is organized in `src/handlers` (API logic), `src/models` (ORM models), and `src/database.rs` (DB connection).
- Migrations are defined in `src/migration.rs`.
- Use `cargo fmt` and `cargo clippy` for linting and formatting.
