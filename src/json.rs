use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterData {
  pub email: String,
  pub username: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenData {
  pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthTokens {
  pub refresh_token: String,
  pub access_token: String,
}