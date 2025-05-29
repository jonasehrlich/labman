use diesel::{connection::DefaultLoadingMode, prelude::*};
use dotenvy::dotenv;
use models::{NewUser, User, UserRole};
use schema::users;
use std::env;

pub mod cli;
pub mod models;
pub mod schema;

pub struct Labman {
    conn: SqliteConnection,
}

impl Labman {
    pub fn new() -> Result<Self, diesel::result::ConnectionError> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let conn = SqliteConnection::establish(&database_url)?;
        Ok(Labman { conn: conn })
    }

    /// Create a user in the database
    pub fn create_user(
        &mut self,
        name: &String,
        role: &models::UserRole,
    ) -> Result<models::User, diesel::result::Error> {
        // use self::schema::user_roles::dsl::*;
        // use crate::schema::users;
        // let new_user = NewUser
        let new_user = NewUser { name, role };
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut self.conn)
    }

    /// Get a user from the database
    pub fn get_user_by_name(
        &mut self,
        name: &String,
    ) -> Result<models::User, diesel::result::Error> {
        use schema::users::dsl::{name as user_name, users};
        users
            .filter(user_name.eq(name))
            .select(models::User::as_select())
            .first(&mut self.conn)
    }

    /// Get the users with a minimum role
    pub fn get_users<'labman>(
        &'labman mut self,
        min_role: &'labman UserRole,
    ) -> Result<
        impl Iterator<Item = Result<User, diesel::result::Error>> + 'labman,
        diesel::result::Error,
    > {
        use schema::users::dsl::{role, users};
        users
            .filter(role.ge(min_role))
            .load_iter::<User, DefaultLoadingMode>(&mut self.conn)
    }

    pub fn delete_user(&mut self, name: &String) -> Option<diesel::result::Error> {
        use schema::users::dsl::{id as user_id, users};
        match self.get_user_by_name(name) {
            Ok(user) => {
                if let Err(err) = diesel::delete(users.filter(user_id.eq(user.id))).execute(&mut self.conn) {
                    Some(err)
                } else {
                    None
                }
            }
            Err(err) => Some(err),
        }
    }
}
