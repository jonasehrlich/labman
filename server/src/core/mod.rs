use deadpool_diesel::sqlite::{Manager, Pool, Runtime};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::error::Error;

pub mod models;
pub mod user;

mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Labman {
    // TODO: Make more generic for other databases
    pool: Pool,
}

impl Labman {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let manager = Manager::new(database_url, Runtime::Tokio1);
        let pool = Pool::builder(manager).build().unwrap();

        let conn = pool.get().await?;
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();

        Ok(Labman { pool })
    }

    pub fn user(&self) -> user::UserManager {
        user::UserManager::new(&self.pool)
    }
}
