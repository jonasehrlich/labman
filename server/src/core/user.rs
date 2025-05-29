use crate::core::models;
use crate::core::schema;
use diesel::{connection::DefaultLoadingMode, prelude::*};

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
        let new_user = models::NewUser { name, role };
        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(models::User::as_returning())
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
        min_role: &'labman models::UserRole,
    ) -> Result<
        impl Iterator<Item = Result<models::User, diesel::result::Error>> + 'labman,
        diesel::result::Error,
    > {
        use schema::users;
        users::table
            .filter(users::role.ge(min_role))
            .load_iter::<models::User, DefaultLoadingMode>(self.conn)
    }

    pub fn delete(&mut self, name: &String) -> Result<(), diesel::result::Error> {
        use schema::users;
        match self.get(name) {
            Ok(user) => {
                if let Err(err) =
                    diesel::delete(users::table.filter(users::id.eq(user.id))).execute(self.conn)
                {
                    Err(err)
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(err),
        }
    }
}
