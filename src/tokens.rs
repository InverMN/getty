use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  exp: usize,
  user_id: String,
}

fn sign_token(user_id: &str, expiration_time: u128) -> String {
  let start = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap();

  let expiration_date = start.as_millis() + expiration_time;

  let claims = Claims {
    exp: expiration_date as usize,
    user_id: user_id.to_string(),
  };

  let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("my_secretly_secreted_secret".as_ref())).unwrap();

  token
}

pub fn sign_refresh_token(user_id: &str) -> String {
  sign_token(user_id, 10 * 24 * 60 * 60 * 1000)
}

pub fn sign_access_token(user_id: &str) -> String {
  sign_token(user_id, 10 * 60 * 1000)
}

pub fn verify_refresh_token(token: &str) -> Option<String> {
  let claims = decode::<Claims>(token, &DecodingKey::from_secret("my_secretly_secreted_secret".as_ref()), &Validation::new(Algorithm::HS256)).unwrap().claims;
  Some(claims.user_id.to_owned())
}