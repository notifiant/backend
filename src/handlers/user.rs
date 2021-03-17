use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;

pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
  Ok(web::block(move || get_all_users(db))
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
  let conn = pool.get().unwrap();
  let items = users.load::<User>(&conn)?;
  Ok(items)
}