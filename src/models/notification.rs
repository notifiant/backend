use crate::schema::notifications;

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