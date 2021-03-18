use super::user::User;
use crate::diesel::{
  insert_into, result::Error as DieselError, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use crate::schema::notifications;
use crate::schema::notifications::dsl::*;
use crate::Pool;
use actix_web::{web};
use serde::{Deserialize, Serialize};

type DbPool = web::Data<Pool>;

#[derive(Debug, Insertable, Deserialize, AsChangeset, Serialize)]
#[table_name = "notifications"]
pub struct NewNotification {
  pub price: f64,
  pub user_id: i32,
}

impl NewNotification {
  fn from(n: NewNotification) -> NewNotification {
    NewNotification {
      price: n.price,
      user_id: 1,
    }
  }
}

#[derive(Debug, Queryable, Serialize, Deserialize, Associations)]
#[table_name = "notifications"]
pub struct Notification {
  pub id: i32,
  pub price: f64,
  pub user_id: i32,
}

impl Notification {
  pub fn find_all(pool: DbPool) -> Result<Vec<Notification>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let items = notifications.load::<Notification>(&conn)?;
    Ok(items)
  }

  pub fn find_by_id(n_id: i32, pool: DbPool) -> Result<Notification, DieselError> {
    let conn = pool.get().unwrap();
    let notification = notifications::table.find(n_id).first(&conn)?;

    Ok(notification)
  }

  pub fn create(notification: NewNotification, pool: DbPool) -> Result<Notification, DieselError> {
    let conn = pool.get().unwrap();
    let new_notification = NewNotification::from(notification);
    println!("{:?}", new_notification);

    let res = insert_into(notifications)
      .values(&new_notification)
      .get_result(&conn)?;

    Ok(res)
  }

  pub fn update(
    n_id: i32,
    notification: NewNotification,
    pool: DbPool,
  ) -> Result<Self, DieselError> {
    let conn = pool.get().unwrap();

    let notification = diesel::update(notifications::table.filter(notifications::id.eq(n_id)))
      .set(notification)
      .get_result(&conn)?;

    Ok(notification)
  }

  pub fn delete(n_id: i32, pool: DbPool,) -> Result<usize, DieselError> {
    let conn = pool.get().unwrap();

    let res = diesel::delete(notifications::table.filter(notifications::id.eq(n_id))).execute(&conn)?;
    Ok(res)
  }
}
