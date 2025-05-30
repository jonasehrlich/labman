use crate::core::models;
use crate::core::schema;
use deadpool_diesel::sqlite::Pool;
use diesel::prelude::*;

pub struct UserManager<'a> {
    pool: &'a Pool,
}

impl<'a> UserManager<'a> {
    pub fn new(pool: &'a Pool) -> Self {
        UserManager { pool }
    }

    /// Create a user in the database
    pub async fn create(
        &self,
        name: &str,
        role: &models::UserRole,
    ) -> Result<models::User, Box<dyn std::error::Error>> {
        use schema::users;

        let name = name.to_string();
        let role = *role;

        let conn = self.pool.get().await?;
        let res = conn
            .interact(move |conn| {
                let new_user = models::NewUser {
                    name: &name,
                    role: &role,
                };
                diesel::insert_into(users::table)
                    .values(new_user)
                    .returning(models::User::as_returning())
                    .get_result(conn)
            })
            .await??;
        Ok(res)
    }

    /// Get a user from the database
    pub async fn get(&self, name: &str) -> Result<models::User, Box<dyn std::error::Error>> {
        use schema::users;
        let conn = self.pool.get().await?;

        let name = name.to_string();
        let res = conn
            .interact(move |conn| {
                users::table
                    .filter(users::name.eq(&name))
                    .select(models::User::as_select())
                    .first(conn)
            })
            .await??;
        Ok(res)
    }

    /// Get the users with a minimum role
    pub async fn iter<'labman>(
        &'labman self,
        min_role: &'labman models::UserRole,
    ) -> Result<
        impl Iterator<Item = Result<models::User, Box<dyn std::error::Error>>> + 'labman,
        Box<dyn std::error::Error>,
    > {
        use schema::users;
        let conn = self.pool.get().await?;
        let min_role = *min_role;
        let res = conn
            .interact(move |conn| {
                users::table
                    .filter(users::role.ge(min_role))
                    .select(models::User::as_select())
                    .load::<models::User>(conn)
            })
            .await??;
        Ok(res.into_iter().map(Ok))
    }

    pub async fn delete(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        use schema::users;
        let conn = self.pool.get().await?;

        let name = name.to_string();
        conn.interact(move |conn| {
            diesel::delete(users::table)
                .filter(users::name.eq(&name))
                .execute(conn)
        })
        .await??;
        Ok(())
    }
}
