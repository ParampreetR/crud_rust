use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement};

pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not provided");

    let db = Database::connect(&database_url).await?;
    Migrator::up(&db, None).await?;

    Ok(db)
}
