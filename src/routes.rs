use crate::database;
use crate::structs;
use crate::tokens;
use crate::config;
use crate::metrics;

use rocket::State;
use rocket_contrib::json::Json;
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};
use database::{save_token, used_token};
use rocket::http::Status;
use structs::{AuthTokens, RefreshTokenData};
use aerospike::Client;
use config::Config;
use metrics::{REGISTRY, HTTP_REQUESTS_TOTAL, REFRESH_ATTEMPTS, LOGOUT_ATTEMPTS, INCOMING_REQUESTS};

#[get("/")]
pub fn index() -> &'static str {
  HTTP_REQUESTS_TOTAL.inc();
  INCOMING_REQUESTS.with_label_values(&["index"]).inc();
  "JWT auth server"
}

#[get("/metrics")]
pub fn metrics() -> Result<String, Status> {
  use prometheus::{Encoder, TextEncoder};
  HTTP_REQUESTS_TOTAL.inc();
  INCOMING_REQUESTS.with_label_values(&["metrics"]).inc();

  let encoder = TextEncoder::new();

  let mut buffer = Vec::new();
  if let Err(error) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
    eprintln!("could not encode service metrics: {}", error);
  };
  let response_service = match String::from_utf8(buffer.clone()) {
    Ok(value) => value,
    Err(error) => {
      eprintln!("could not parse service metric as String: {}", error);
      String::default()
    },
  };
  buffer.clear();

  let mut buffer = Vec::new();
  if let Err(error) = encoder.encode(&prometheus::gather(), &mut buffer) {
    eprintln!("could not encode Prometheus metrics: {}", error);
  };
  let response_prometheus = match String::from_utf8(buffer.clone()) {
    Ok(value) => value,
    Err(error) => {
      eprintln!("could not parse Prometheus metric as String: {}", error);
      String::default()
    }
  };
  buffer.clear();

  Ok(response_service + &response_prometheus)
}

#[post("/refresh", data="<refresh_token_data>")]
pub fn refresh(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Result<Json<AuthTokens>, Status> {
  HTTP_REQUESTS_TOTAL.inc();
  INCOMING_REQUESTS.with_label_values(&["refresh"]).inc();
  if used_token(&client, &refresh_token_data.refresh_token, &config).unwrap() {
    REFRESH_ATTEMPTS.with_label_values(&["failure"]).inc();
    return Err(Status::Unauthorized);
  }

  let user_id = match verify_refresh_token(&refresh_token_data.refresh_token, &config) {
    Some(value) => value,
    None => {
      REFRESH_ATTEMPTS.with_label_values(&["failure"]).inc();
      return Err(Status::Unauthorized);
    },
  };
  
  save_token(&client, &refresh_token_data.refresh_token, &config);
  
  let refresh_token = sign_refresh_token(&user_id, &config);
  let access_token = sign_access_token(&user_id, &config); 
  
  let auth_tokens = AuthTokens {
    refresh_token,
    access_token,
  };
  
  REFRESH_ATTEMPTS.with_label_values(&["success"]).inc();
  Ok(Json(auth_tokens))
}

#[post("/logout", data="<refresh_token_data>")]
pub fn logout(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Status {
  HTTP_REQUESTS_TOTAL.inc();
  INCOMING_REQUESTS.with_label_values(&["logout"]).inc();
  if used_token(&client, &refresh_token_data.refresh_token, &config).unwrap() {
    LOGOUT_ATTEMPTS.with_label_values(&["failure"]).inc();
    return Status::Unauthorized;
  } else {
    LOGOUT_ATTEMPTS.with_label_values(&["success"]).inc();
    save_token(&client, &refresh_token_data.refresh_token, &config);
  }
  Status::Ok
}

#[get("/get")]
pub fn get_token(config: State<Config>) -> Json<AuthTokens> {
  HTTP_REQUESTS_TOTAL.inc();
  let user_id = "custom_id".to_owned();

  let refresh_token = sign_refresh_token(&user_id, &config);
  let access_token = sign_access_token(&user_id, &config); 

  let auth_tokens = AuthTokens {
    refresh_token,
    access_token,
  };

  Json(auth_tokens)
}