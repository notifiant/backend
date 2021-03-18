use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub username: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
  pub hash: String,
}

#[derive(Deserialize, Debug)]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub email: &'a str,
  pub password: &'a str,
}