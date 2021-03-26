use actix_web::{web, HttpRequest, HttpResponse};
use log::info;

use crate::{model::UserRequest, LightError, LightErrorType, LightState};

pub async fn regen_user(
  req: HttpRequest,
  data: web::Json<UserRequest>,
  state: web::Data<LightState>
) -> Result<HttpResponse, LightError> {
  match req.headers().get("authorization") {
    Some(header) => {
      let auth = header.to_str().expect("couldn't str auth");
      if auth != state.config.admin_key {
        return Err(LightError {
          r#type: LightErrorType::AuthFailed
        });
      }
      match state.pg.regen_user(&data.name).await {
        Ok(user) => {
          info!("user regenned {}", user.name);
          Ok(HttpResponse::Ok().body(user.auth))
        }
        Err(e) => Err(e)
      }
    }

    None => Err(LightError {
      r#type: LightErrorType::AuthFailed
    })
  }
}
