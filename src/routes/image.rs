use actix_files::NamedFile;
use actix_web::web;

use crate::{LightError, LightErrorType, LightState};

pub async fn image(
  params: web::Path<String>,
  state: web::Data<LightState>
) -> Result<NamedFile, LightError> {
  match state
    .pg
    .image_exists(params.to_string())
    .await
    .expect("couldn't query to pg")
  {
    true => Ok(
      NamedFile::open(format!("{}/{}", state.config.uploads_dir, params))
        .expect("couldn't open file")
    ),
    false => Err(LightError {
      r#type: LightErrorType::ImageNoExist
    })
  }
}
