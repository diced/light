use serde::{Deserialize, Serialize};
use std::{error::Error, fs::*};

#[derive(Deserialize, Serialize, Debug)]
pub struct ParsableConfig {
  pub host: Option<String>,
  pub postgres_uri: Option<String>,
  pub admin_key: Option<String>,
  pub uploads_dir: Option<String>,
  pub uploads_route: Option<String>,
  pub token_length: Option<usize>,
  pub file_length: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Config {
  pub host: String,
  pub postgres_uri: String,
  pub admin_key: String,
  pub uploads_dir: String,
  pub uploads_route: String,
  pub token_length: usize,
  pub file_length: usize,
}

impl Config {
  pub fn parse() -> Result<Self, Box<dyn Error>> {
    let content = read_to_string("light.toml").unwrap_or(String::from(""));
    let decoded: ParsableConfig = toml::from_str(&content)?;
    Ok(Config {
      host: decoded
        .host
        .unwrap_or_else(|| String::from("0.0.0.0:8000")),
      postgres_uri: decoded.postgres_uri
        .unwrap_or_else(|| String::from("postgresql://light:light@postgres/light")),
      admin_key: decoded.admin_key
      .unwrap_or_else(|| String::from("1234")),
      uploads_dir: decoded
        .uploads_dir
        .unwrap_or_else(|| String::from("./uploads")),
      uploads_route: decoded.uploads_route.unwrap_or_else(|| String::from("/i")),
      token_length: decoded.token_length.unwrap_or(12),
      file_length: decoded.file_length.unwrap_or(5),
    })
  }
}
