use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::{borrow::BorrowMut, error::Error};

pub mod models;
pub mod user;

mod schema;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Labman {
    // TODO: Make generic
    conn: SqliteConnection,
}

impl Labman {
    pub fn new(database_url: &str) -> Result<Self, diesel::result::ConnectionError> {
        let conn = SqliteConnection::establish(database_url)?;
        Ok(Labman { conn })
    }

    pub fn run_migrations(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.conn.run_pending_migrations(MIGRATIONS)?;
        Ok(())
    }

    pub fn user(&mut self) -> user::UserManager {
        user::UserManager::new(self.conn.borrow_mut())
    }
}
