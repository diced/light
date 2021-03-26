use serde::{Deserialize, Serialize};

use crate::random_chars;

#[derive(Debug, Clone, Deserialize)]
pub struct UserRequest {
  pub name: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LightUser {
  pub name: String,
  pub auth: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LightImage {
  pub file: String,
  pub user: String
}

impl LightUser {
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      auth: random_chars(12)
    }
  }
}
