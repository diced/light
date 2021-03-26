use actix_web::{HttpRequest, HttpResponse, web};
use log::info;

use crate::{LightErrorType, LightState, LightError, model::UserRequest};

pub async fn create_user(
  req: HttpRequest,
  data: web::Json<UserRequest>,
  state: web::Data<LightState>,
) -> Result<HttpResponse, LightError> {
  match req.headers().get("authorization") {
    Some(header) => {
      let auth = header.to_str().expect("couldn't str auth");
      if auth != state.config.admin_key {
        Err(LightError {
          r#type: LightErrorType::AuthFailed,
        })
      } else {
        match state.pg.create_user(&data.name).await {
          Ok(user) => {
            info!("new user created {}", user.name);
            Ok(HttpResponse::Ok().body(user.auth))
          },
          Err(e) => Err(e),
        }
      }
    }
    None => Err(LightError {
      r#type: LightErrorType::AuthFailed,
    }),
  }
}