use crate::config;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};
use config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  exp: usize,
  user_id: String,
}

fn sign_token(user_id: &str, expiration_time: u128, config: &Config) -> String {
  let start = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap();

  let expiration_date = start.as_millis() + expiration_time;

  let claims = Claims {
    exp: expiration_date as usize,
    user_id: user_id.to_string(),
  };

  let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(config.secret.as_ref())).unwrap();

  token
}

pub fn sign_refresh_token(user_id: &str, config: &Config) -> String {
  sign_token(user_id, config.refresh_token_exp, config)
}

pub fn sign_access_token(user_id: &str, config: &Config) -> String {
  sign_token(user_id, config.access_token_exp, config)
}

pub fn verify_refresh_token(token: &str, config: &Config) -> Option<String> {
  match decode::<Claims>(token, &DecodingKey::from_secret(config.secret.as_ref()), &Validation::new(Algorithm::HS256)) {
    Ok(value) => Some(value.claims.user_id),
    Err(_) => None,
  }
}