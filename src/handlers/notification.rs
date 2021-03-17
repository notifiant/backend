use crate::models::notification::{NewNotification, Notification};
use crate::Pool;
use actix_web::{web, Error, HttpResponse};

pub async fn create_notification(db: web::Data<Pool>, item: web::Json<NewNotification>) -> Result<HttpResponse, Error> {
  let notification = Notification::create(item, db)
    .await
    .map(|n| HttpResponse::Created().json(n))
    .map_err(|_| HttpResponse::BadRequest())?;

  Ok(notification)
}