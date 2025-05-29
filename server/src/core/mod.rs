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
    pub fn new(database_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut conn = SqliteConnection::establish(database_url)?;
        conn.run_pending_migrations(MIGRATIONS)?;
        Ok(Labman { conn })
    }

    pub fn user(&mut self) -> user::UserManager {
        user::UserManager::new(self.conn.borrow_mut())
    }
}
