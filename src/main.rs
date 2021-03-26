use std::{fs::File, io::Write, sync::Arc};

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use futures::{StreamExt, TryStreamExt};
use light::{
  model::UserRequest,
  postgres::Postgres,
  random_chars, Config, LightErrorType, LightResult, LightState, LightWebError,
};

#[post("/api/create_user")]
async fn create_user(
  req: HttpRequest,
  data: web::Json<UserRequest>,
  state: web::Data<LightState>,
) -> Result<HttpResponse, LightWebError> {
  match req.headers().get("authorization") {
    Some(header) => {
      let auth = header.to_str().expect("couldn't str auth");
      if auth != state.config.admin_key {
        Err(LightWebError {
          r#type: LightErrorType::AuthFailed,
        })
      } else {
        match state.pg.create_user(&data.name).await {
          Ok(user) => Ok(HttpResponse::Ok().body(user.auth)),
          Err(e) => Err(e.to_web()),
        }
      }
    }
    None => Err(LightWebError {
      r#type: LightErrorType::AuthFailed,
    }),
  }
}

#[post("/upload")]
async fn upload(
  req: web::HttpRequest,
  state: web::Data<LightState>,
  mut payload: Multipart,
) -> Result<HttpResponse, LightWebError> {
  let auth_header = req.headers().get("authorization");
  let auth = match auth_header.clone() {
    None => false,
    Some(value) => state
      .pg
      .check_token(value.to_str().expect("couldn't str auth"))
      .await
      .unwrap(),
  };

  if !auth {
    return Err(LightWebError {
      r#type: LightErrorType::AuthFailed,
    });
  }

  let user = state
    .pg
    .user_by_auth(auth_header.unwrap().to_str().unwrap())
    .await
    .expect("no user found");

  let mut field = payload
    .try_next()
    .await
    .expect("no multipart field provided")
    .expect("no multipart field provided");

  let content_disposition = field.content_disposition().unwrap();
  let ext = content_disposition
    .get_filename()
    .unwrap()
    .split('.')
    .last()
    .unwrap_or_else(|| "bin");

  let chars = random_chars(state.config.file_length);

  let filename = format!("{}.{}", chars, ext);

  let mut f = File::create(format!("{}/{}", state.config.uploads_dir, filename))
    .expect("couldn't write to file");

  while let Some(chunk) = field.next().await {
    let data = chunk.unwrap();
    f.write_all(&data).expect("couldn't write to file");
  }

  let file_size = f.metadata().expect("couldn't open file").len();

  if file_size == 0 {
    std::fs::remove_file(format!("{}/{}", state.config.uploads_dir, filename))
      .expect("couldn't remove file");

    return Err(LightWebError {
      r#type: LightErrorType::NoBytes,
    });
  }

  state
    .pg
    .create_image(filename.clone(), user)
    .await
    .expect("couldn't save image");

  Ok(HttpResponse::Ok().body(filename))
}

async fn delete(
  req: web::HttpRequest,
  params: web::Path<String>,
  state: web::Data<LightState>,
) -> Result<HttpResponse, LightWebError> {
  let auth_header = req.headers().get("authorization");
  let auth = match auth_header.clone() {
    None => false,
    Some(value) => state
      .pg
      .check_token(value.to_str().expect("couldn't str auth"))
      .await
      .unwrap(),
  };

  if !auth {
    return Err(LightWebError {
      r#type: LightErrorType::AuthFailed,
    });
  }

  let user = state
    .pg
    .user_by_auth(auth_header.unwrap().to_str().unwrap())
    .await
    .expect("no user found");

  let some_image = state.pg.get_image(params.clone(), user)
    .await
    .expect("couldn't get images");

  if some_image.is_none() {
    return Err(LightWebError {
      r#type: LightErrorType::ImageNoExist
    });
  }

  let image = some_image.expect("couldn't get image");

  std::fs::remove_file(format!("{}/{}", state.config.uploads_dir, image.file)).expect("couldn't remove file");
  state.pg.delete_image(image).await;

  Ok(HttpResponse::Ok().body("ok"))
}

async fn image(params: web::Path<String>, state: web::Data<LightState>) -> Result<NamedFile, LightWebError> {
  match state.pg.image_exists(params.to_string()).await.expect("couldn't query to pg") {
    true => Ok(NamedFile::open(format!("{}/{}", state.config.uploads_dir, params)).expect("couldn't open file")),
    false => Err(LightWebError {
      r#type: LightErrorType::ImageNoExist
    })
  }
}

#[actix_web::main]
async fn main() -> LightResult<()> {
  let config = Config::parse().expect("can't read config");

  let pg = Arc::new(Postgres::connect(&config.postgres_uri).await?);

  pg.query(
    "CREATE TABLE IF NOT EXISTS light_users (id SERIAL PRIMARY KEY NOT NULL, data JSONB)",
    &[],
  )
  .await?;

  pg.query(
    "CREATE TABLE IF NOT EXISTS light_images (id SERIAL PRIMARY KEY NOT NULL, data JSONB)",
    &[],
  )
  .await?;

  let http_config = config.clone();
  HttpServer::new(move || {
    App::new()
      .data(LightState {
        pg: pg.clone(),
        config: http_config.clone(),
      })
      .service(upload)
      .service(create_user)
      .route(format!("{}/{{name}}", http_config.uploads_route).as_str(), web::delete().to(delete))
      .route(format!("{}/{{name}}", http_config.uploads_route).as_str(), web::get().to(image))
    })
    .bind(config.host)?
    .run()
    .await?;

    Ok(())
}