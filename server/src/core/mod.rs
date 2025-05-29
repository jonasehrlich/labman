use diesel::prelude::*;
use dotenvy::dotenv;
use std::borrow::BorrowMut;
use std::env;

pub mod models;
pub mod user;

mod schema;

pub struct Labman {
    // TODO: Make generic
    conn: SqliteConnection,
}

impl Labman {
    pub fn new() -> Result<Self, diesel::result::ConnectionError> {
        // TODO: remove dotenv handling from here
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)?;
        Ok(Labman { conn: conn })
    }

    pub fn user(&mut self) -> user::UserManager {
        user::UserManager::new(self.conn.borrow_mut())
    }
}
