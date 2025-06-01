pub mod entity;
pub mod user;
use sea_orm_migration::MigratorTrait;

pub struct Labman {
    db: sea_orm::DatabaseConnection,
}

impl Labman {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sea_orm::Database::connect(database_url).await?;
        // TODO: Apply migrations
        migration::Migrator::up(&db, None).await?;

        Ok(Labman { db })
    }

    pub fn user(&self) -> user::UserManager {
        user::UserManager::new(&self.db)
    }
}
