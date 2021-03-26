use std::{error, fmt, sync::Arc};

use postgres::Postgres;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

mod config;
pub mod model;
pub mod postgres;
pub mod routes;

pub use config::{Config, ParsableConfig};

pub type LightResult<T> = std::result::Result<T, Box<dyn error::Error + Send + Sync>>;
pub type LightPgResult<T> = std::result::Result<T, LightError>;
pub type LightWebResult<T> = actix_web::Result<T, Box<dyn error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct LightState {
  pub pg: Arc<Postgres>,
  pub config: Config
}

#[derive(Debug, Clone)]
pub enum LightErrorType {
  ConnectToPgFailed,
  UserExists,
  AuthFailed,
  NoBytes,
  ImageNoExist
}

#[derive(Debug, Clone)]
pub struct LightError {
  pub r#type: LightErrorType
}

impl fmt::Display for LightError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let msg = match self.r#type {
      LightErrorType::ConnectToPgFailed => "couldln't connect to pg",
      LightErrorType::UserExists => "user exists",
      LightErrorType::AuthFailed => "auth failed",
      LightErrorType::NoBytes => "no bytes",
      LightErrorType::ImageNoExist => "image doesnt exist"
    };

    write!(f, "{}", msg)
  }
}

impl error::Error for LightError {}
impl actix_web::error::ResponseError for LightError {}

pub fn random_chars(len: usize) -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}
