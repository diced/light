use actix_web::{HttpResponse, web};
use log::{debug, info};

use crate::{LightState, LightError, LightErrorType};

pub async fn delete(
  req: web::HttpRequest,
  params: web::Path<String>,
  state: web::Data<LightState>,
) -> Result<HttpResponse, LightError> {
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
    return Err(LightError {
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
    return Err(LightError {
      r#type: LightErrorType::ImageNoExist
    });
  }

  let image = some_image.expect("couldn't get image");

  debug!("attempting to delete image {}", image.file);
  std::fs::remove_file(format!("{}/{}", state.config.uploads_dir, image.file)).expect("couldn't remove file");
  state.pg.delete_image(image.clone()).await;
  info!("to deleted image {}", image.file);

  Ok(HttpResponse::Ok().body("deleted"))
}