use tokio_postgres::{
  types::{Json, ToSql},
  Client, NoTls, Row,
};

use crate::{
  model::{LightImage, LightUser},
  LightError, LightErrorType, LightPgResult, LightResult,
};

#[derive(Debug)]
pub struct Postgres {
  client: Client,
}

impl Postgres {
  pub async fn connect(uri: &str) -> LightResult<Self> {
    let (client, connection) = tokio_postgres::connect(uri, NoTls).await?;

    actix_rt::spawn(async move {
      if let Err(_) = connection.await {
        eprintln!("no conet");
      }
    });

    Ok(Self { client })
  }

  pub async fn query(
    &self,
    statement: &str,
    params: &[&(dyn ToSql + Sync)],
  ) -> LightResult<Vec<Row>> {
    Ok(self.client.query(statement, params).await?)
  }

  pub async fn create_user(&self, name: &str) -> LightPgResult<LightUser> {
    match self.user_exists(name).await.expect("exist not happen") {
      Some(_) => Err(LightError(LightErrorType::UserExists)),
      None => {
        let user = LightUser::new(name);

        self
          .query(
            "INSERT INTO light_users (data) VALUES ($1)",
            &[&Json::<LightUser>(user.clone())],
          )
          .await
          .expect("couldn't query");

        Ok(user)
      }
    }
  }

  pub async fn create_image(
    &self,
    filename: impl Into<String>,
    user: LightUser,
  ) -> LightPgResult<LightImage> {
    let image = LightImage {
      file: filename.into(),
      user: user.name,
    };

    self
      .query(
        "INSERT INTO light_images (data) VALUES ($1)",
        &[&Json::<LightImage>(image.clone())],
      )
      .await
      .expect("couldn't query");

    Ok(image)
  }

  pub async fn get_image(
    &self,
    filename: impl Into<String>,
    user: LightUser,
  ) -> LightPgResult<Option<LightImage>> {
    let res = self
      .query(
        "SELECT (data) FROM light_images WHERE data->'file' @> $1 AND data->'user' @> $2",
        &[&Json::<String>(filename.into()), &Json::<String>(user.name)],
      )
      .await
      .expect("couldn't query");

      if res.len() >= 1 {
        let col: Json<LightImage> = res.get(0).expect("").get(0);
  
        Ok(Some(col.0))
      } else {
        Ok(None)
      }
  }

  pub async fn image_exists(
    &self,
    filename: impl Into<String>
  ) -> LightPgResult<bool> {
    let res = self
      .query(
        "SELECT (data) FROM light_images WHERE data->'file' @> $1",
        &[&Json::<String>(filename.into())],
      )
      .await
      .expect("couldn't query");

      if res.len() >= 1 {
        Ok(true)
      } else {
        Ok(false)
      }
  }

  pub async fn delete_image(&self, image: LightImage) {
    self
      .query(
        "DELETE FROM light_images WHERE data->'file' @> $1 AND data->'user' @> $2",
        &[&Json::<String>(image.file), &Json::<String>(image.user)],
      )
      .await
      .expect("couldn't query");
  }

  pub async fn user_exists(&self, name: &str) -> LightResult<Option<LightUser>> {
    let res = self
      .query(
        "SELECT (data) FROM light_users WHERE data->'name' @> $1",
        &[&Json::<String>(name.into())],
      )
      .await?;

    if res.len() >= 1 {
      let col: Json<LightUser> = res.get(0).expect("").get(0);

      Ok(Some(col.0))
    } else {
      Ok(None)
    }
  }

  pub async fn check_token(&self, token: &str) -> LightResult<bool> {
    let res = self
      .query(
        "SELECT * FROM light_users WHERE data->'auth' @> $1",
        &[&Json::<String>(token.into())],
      )
      .await?;

    if res.len() >= 1 {
      Ok(true)
    } else {
      Ok(false)
    }
  }

  pub async fn user_by_auth(&self, token: &str) -> Option<LightUser> {
    let res = self
      .query(
        "SELECT (data) FROM light_users WHERE data->'auth' @> $1",
        &[&Json::<String>(token.into())],
      )
      .await
      .expect("couldn't get user");

    if res.len() >= 1 {
      let col: Json<LightUser> = res.get(0).unwrap().get(0);

      Some(col.0)
    } else {
      None
    }
  }
}
