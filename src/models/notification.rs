use crate::schema::notifications;
use crate::schema::notifications::dsl::*;
use diesel::dsl::not;
use serde::{Deserialize, Serialize};
use actix_web::{web, Error, HttpResponse};
use crate::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl, result::Error as DieselError};

#[derive(Queryable)]
pub struct Notification {
  pub id: i32,
  pub price: f64,
}

#[derive(Insertable)]
#[table_name="notifications"]
pub struct NewNotification {
  pub price: Option<f64>,
}

impl Notification {
  pub async fn find_all(pool: web::Data<Pool>) -> Result<Vec<Notification>, DieselError> {
    let conn = pool.get().unwrap();
    let items = notifications.load::<Notification>(&conn)?;
    Ok(items)
  }

  pub async fn find_by_id(nid: i32, pool: web::Data<Pool>) -> Result<Notification, DieselError> {
    let conn = pool.get().unwrap();
    let notification = notifications::table.find(nid).first(&conn)?;

    Ok(notification)
  }
}