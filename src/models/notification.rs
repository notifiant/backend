use crate::schema::notifications;
use crate::schema::notifications::dsl::*;
use serde::{Deserialize, Serialize};
use actix_web::{web, Error, HttpResponse};
use crate::Pool;
use crate::diesel::{QueryDsl, RunQueryDsl, result::Error as DieselError, insert_into};

type DbPool = web::Data<Pool>;

#[derive(Queryable)]
pub struct Notification {
  pub id: i32,
  pub price: f64,
}

#[derive(Insertable)]
#[table_name="notifications"]
pub struct NewNotification {
  pub price: f64,
}

impl Notification {
  pub async fn find_all(pool: DbPool) -> Result<Vec<Notification>, DieselError> {
    let conn = pool.get().unwrap();
    let items = notifications.load::<Notification>(&conn)?;
    Ok(items)
  }

  pub async fn find_by_id(nid: i32, pool: DbPool) -> Result<Notification, DieselError> {
    let conn = pool.get().unwrap();
    let notification = notifications::table.find(nid).first(&conn)?;

    Ok(notification)
  }

  pub async fn create(notification: web::Json<NewNotification>, pool: DbPool) -> Result<Notification, DieselError> {
    let conn = pool.get().unwrap();
    let new_notification = NewNotification {
      price: notification.price,
    };
    let res = insert_into(notifications::table).values(new_notification).get_result(&conn)?;

    Ok(res)
  }
}