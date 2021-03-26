use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use light::{Config, LightResult, LightState, postgres::Postgres, routes::{create_user, delete, delete_user, image, regen_user, upload}};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[actix_web::main]
async fn main() -> LightResult<()> {
  SimpleLogger::new()
    .with_module_level("tokio_util", LevelFilter::Off)
    .with_module_level("tokio_postgres", LevelFilter::Off)
    .with_module_level("mio", LevelFilter::Off)
    .with_module_level("actix_server::worker", LevelFilter::Off)
    .with_module_level("actix_http::h1", LevelFilter::Off)
    .init()?;

  let config = Config::parse().expect("can't read config");

  let pg = Arc::new(Postgres::connect(&config.postgres_uri).await?);

  pg.query(
    "CREATE TABLE IF NOT EXISTS light_users (id SERIAL PRIMARY KEY NOT NULL, data JSONB)",
    &[]
  )
  .await?;
  info!("CREATE TABLE IF NOT EXISTS light_users");

  pg.query(
    "CREATE TABLE IF NOT EXISTS light_images (id SERIAL PRIMARY KEY NOT NULL, data JSONB)",
    &[]
  )
  .await?;
  info!("CREATE TABLE IF NOT EXISTS light_images");

  let http_config = config.clone();
  HttpServer::new(move || {
    App::new()
      .data(LightState {
        pg: pg.clone(),
        config: http_config.clone()
      })
      .route("/upload", web::post().to(upload))
      .route("/user", web::post().to(create_user))
      .route("/user", web::delete().to(delete_user))
      .route("/user", web::patch().to(regen_user))
      .route(
        format!("{}/{{name}}", http_config.uploads_route).as_str(),
        web::delete().to(delete)
      )
      .route(
        format!("{}/{{name}}", http_config.uploads_route).as_str(),
        web::get().to(image)
      )
  })
  .bind(config.host)?
  .run()
  .await?;

  Ok(())
}
