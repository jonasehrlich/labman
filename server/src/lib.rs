use diesel::{connection::DefaultLoadingMode, prelude::*};
use dotenvy::dotenv;
use models::{NewUser, User, UserRole};
use std::borrow::BorrowMut;
use std::env;

pub mod cli;
pub mod models;
pub mod schema;

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

    pub fn user(&mut self) -> UserManager {
        UserManager::new(self.conn.borrow_mut())
    }
}

pub struct UserManager<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> UserManager<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        UserManager { conn }
    }

    /// Create a user in the database
    pub fn create(
        &mut self,
        name: &String,
        role: &models::UserRole,
    ) -> Result<models::User, diesel::result::Error> {
        use schema::users;
        let new_user = NewUser { name, role };
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(self.conn)
    }

    /// Get a user from the database
    pub fn get(&mut self, name: &String) -> Result<models::User, diesel::result::Error> {
        use schema::users;
        users::table
            .filter(users::name.eq(name))
            .select(models::User::as_select())
            .first(self.conn)
    }

    /// Get the users with a minimum role
    pub fn iter<'labman>(
        &'labman mut self,
        min_role: &'labman UserRole,
    ) -> Result<
        impl Iterator<Item = Result<User, diesel::result::Error>> + 'labman,
        diesel::result::Error,
    > {
        use schema::users;
        users::table
            .filter(users::role.ge(min_role))
            .load_iter::<User, DefaultLoadingMode>(self.conn)
    }

    pub fn delete(&mut self, name: &String) -> Option<diesel::result::Error> {
        use schema::users;
        match self.get(name) {
            Ok(user) => {
                if let Err(err) =
                    diesel::delete(users::table.filter(users::id.eq(user.id))).execute(self.conn)
                {
                    Some(err)
                } else {
                    None
                }
            }
            Err(err) => Some(err),
        }
    }
}
