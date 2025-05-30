use crate::core::schema;
use clap::ValueEnum;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{self, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::{backend::Backend, prelude::*};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

#[repr(i32)]
#[derive(
    Debug,
    Display,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    ValueEnum,
    EnumIter,
    EnumString,
    FromSqlRow,
    AsExpression,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = Integer)]
pub enum UserRole {
    /// Administrator role, highest level of access
    Administrator = 100,
    /// Developer role, medium level of access
    Developer = 50,
    /// Reporter role, read-only access
    Reporter = 20,
}

impl UserRole {
    /// Create a UserRole from an integer value
    pub fn from_level(level: i32) -> Option<Self> {
        match level {
            x if x == UserRole::Administrator as i32 => Some(UserRole::Administrator),
            x if x == UserRole::Developer as i32 => Some(UserRole::Developer),
            x if x == UserRole::Reporter as i32 => Some(UserRole::Reporter),
            _ => None,
        }
    }

    /// Get the lowest user role
    pub fn min() -> Self {
        UserRole::iter().min().unwrap()
    }

    /// Get the highest user role
    pub fn max() -> Self {
        UserRole::iter().max().unwrap()
    }
}

impl<DB> ToSql<Integer, DB> for UserRole
where
    DB: Backend,
    i32: ToSql<Integer, DB>,
{
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> serialize::Result {
        match self {
            UserRole::Administrator => (UserRole::Administrator as i32).to_sql(out),
            UserRole::Developer => (UserRole::Developer as i32).to_sql(out),
            UserRole::Reporter => (UserRole::Reporter as i32).to_sql(out),
        }
    }
}

impl<DB> FromSql<Integer, DB> for UserRole
where
    DB: Backend,
    i32: FromSql<Integer, DB>,
{
    fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let level = i32::from_sql(bytes)?;
        UserRole::from_level(level).ok_or_else(|| format!("Invalid role level: {}", level).into())
    }
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub role: &'a UserRole,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub role: UserRole,
}
