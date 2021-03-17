#[macro_use]
extern crate diesel;

mod handlers;
mod models;
mod schema;

use actix_web::{dev::ServiceRequest, web, App, Error, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn api_conf(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/user")
      .route(web::get().to(handlers::user::get_users))
  );
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  std::env::set_var("RUST_LOG", "actix_web=debug");
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");


  // create db connection pool
  let manager = ConnectionManager::<PgConnection>::new(database_url);
  let pool: Pool = r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.");

  // Start http server
  HttpServer::new(move || {
    App::new()
      .data(pool.clone())
      .service(
        web::scope("/api").configure(api_conf)
      )
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}