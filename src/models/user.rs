use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
  pub password: String,
}

#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub created_at: chrono::NaiveDateTime,
  pub password: &'a str,
}