use std::{fs::File, io::Write};

use actix_multipart::Multipart;
use actix_web::{web, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use log::{debug, info};

use crate::{random_chars, LightError, LightErrorType, LightState};

pub async fn upload(
  req: web::HttpRequest,
  state: web::Data<LightState>,
  mut payload: Multipart
) -> Result<HttpResponse, LightError> {
  let auth_header = req.headers().get("authorization");
  let auth = match auth_header.clone() {
    None => false,
    Some(value) => state
      .pg
      .check_token(value.to_str().expect("couldn't str auth"))
      .await
      .unwrap()
  };

  if !auth {
    return Err(LightError {
      r#type: LightErrorType::AuthFailed
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

  debug!("recieved multipart body");

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
    debug!("writing chunk {} len", data.len());
    f.write_all(&data).expect("couldn't write to file");
  }

  let file_size = f.metadata().expect("couldn't open file").len();

  if file_size == 0 {
    std::fs::remove_file(format!("{}/{}", state.config.uploads_dir, filename))
      .expect("couldn't remove file");

    debug!("file {} had no bytes, deleting...", filename);

    return Err(LightError {
      r#type: LightErrorType::NoBytes
    });
  }

  state
    .pg
    .create_image(filename.clone(), user)
    .await
    .expect("couldn't save image");

  info!("saved image {}", filename);

  Ok(HttpResponse::Ok().body(filename))
}
