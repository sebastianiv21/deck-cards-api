use sea_orm::{DatabaseConnection, Database, DbErr};

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::connect(&database_url).await
}

pub async fn create_database_if_not_exists() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: implement helper function for database setup
    println!("Database connection will be established");
    Ok(())
}