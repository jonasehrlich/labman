use crate::core::entity;
use sea_orm::*;

pub struct UserManager<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserManager<'a> {
    pub fn new(db: &'a sea_orm::DatabaseConnection) -> Self {
        UserManager { db }
    }

    /// Create a user in the database
    pub async fn create(
        &self,
        name: &str,
        role: &entity::user::UserRole,
    ) -> Result<entity::user::Model, anyhow::Error> {
        let user = entity::user::ActiveModel {
            name: Set(name.to_owned()),
            role: Set(role.to_owned()),
            ..Default::default() // all other attributes are `NotSet`
        };

        let user = entity::user::Entity::insert(user).exec(self.db).await?;
        Ok(entity::user::Model {
            id: user.last_insert_id,
            name: name.to_owned(),
            role: role.to_owned(),
        })
    }

    /// Get a user by name
    pub async fn get_by_name(
        &self,
        name: &str,
    ) -> Result<Option<entity::user::Model>, anyhow::Error> {
        let u: Option<entity::user::Model> = entity::user::Entity::find()
            .filter(entity::user::Column::Name.eq(name))
            .one(self.db)
            .await?;
        Ok(u)
    }

    /// Get a by ID
    pub async fn get_by_id(&self, id: i32) -> Result<Option<entity::user::Model>, anyhow::Error> {
        let u: Option<entity::user::Model> =
            entity::user::Entity::find_by_id(id).one(self.db).await?;
        Ok(u)
    }

    /// Get the users with a minimum role
    pub async fn list(
        &self,
        min_role: &entity::user::UserRole,
    ) -> Result<Vec<entity::user::Model>, anyhow::Error> {
        let users = entity::user::Entity::find()
            .filter(entity::user::Column::Role.gte(min_role.to_owned()))
            .all(self.db)
            .await?;
        Ok(users)
    }

    pub async fn delete(&self, id: i32) -> Result<(), anyhow::Error> {
        match entity::user::Entity::delete_by_id(id)
            .exec(self.db)
            .await?
            .rows_affected
        {
            1 => Ok(()),
            _ => Err(sea_orm::DbErr::RecordNotFound(format!("No record with id {}", id)).into()),
        }
    }
}
