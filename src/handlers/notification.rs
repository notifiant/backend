use crate::models::notification::{NewNotification, Notification};
use crate::Pool;
use actix_web::{Error, HttpResponse, delete, get, post, put, web, error::BlockingError};

#[post("/notification")]
pub async fn create_notification(
  item: web::Json<NewNotification>,
  db: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
  let notification = web::block(move || Notification::create(item.into_inner(), db)).await;

  println!("{:?}", notification);

  match notification {
    Ok(notification) => Ok(HttpResponse::Created().json(notification)),
    Err(err) => match err {
      BlockingError::Error(_) => Err(HttpResponse::InternalServerError())?,
      BlockingError::Canceled => Err(todo!("add err")),
    },
  }
}

#[get("/notifications")]
pub async fn get_all_notifications(db: web::Data<Pool>) -> HttpResponse {
  let xs = web::block(move || Notification::find_all(db)).await;

  match xs {
    Ok(xs) => HttpResponse::Ok().json(xs),
    _ => HttpResponse::InternalServerError().await.unwrap(),
  }
}

#[get("/notification/{id}")]
pub async fn get_notification(id: web::Path<i32>, db: web::Data<Pool>) -> HttpResponse {
  let n = web::block(move || Notification::find_by_id(id.into_inner(), db)).await;

  match n {
    Ok(n) => HttpResponse::Ok().json(n),
    _ => HttpResponse::InternalServerError().await.unwrap(),
  }
}

#[put("/notification/{id}")]
pub async fn update_notification(
  id: web::Path<i32>,
  item: web::Json<NewNotification>,
  db: web::Data<Pool>,
) -> HttpResponse {
  let n = web::block(move || Notification::update(id.into_inner(), item.into_inner(), db)).await;

  match n {
    Ok(n) => HttpResponse::Ok().json(n),
    _ => HttpResponse::InternalServerError().await.unwrap(),
  }
}

#[delete("/notification/{id}")]
pub async fn delete_notification(id: web::Path<i32>, db: web::Data<Pool>) -> Result<HttpResponse, Error> {
  let res = web::block(move || Notification::delete(id.into_inner(), db)).await;

  match res {
    Ok(_) => Ok(HttpResponse::NoContent().await.unwrap()),
    Err(err) => match err {
      BlockingError::Canceled => Err(HttpResponse::InternalServerError())?,
      BlockingError::Error(_) => Err(todo!("add err")),
    }
  }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(create_notification);
  cfg.service(get_all_notifications);
  cfg.service(get_notification);
  cfg.service(update_notification);
  cfg.service(delete_notification);
}
