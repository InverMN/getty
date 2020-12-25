use crate::database;
use crate::structs;
use crate::tokens;
use crate::config;

use rocket::State;
use rocket_contrib::json::Json;
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};
use database::{save_token, used_token};
use rocket::http::Status;
use structs::{AuthTokens, RefreshTokenData};
use aerospike::Client;
use config::Config;

#[get("/")]
pub fn index() -> &'static str {
  "JWT auth server"
}

#[post("/refresh", data="<refresh_token_data>")]
pub fn refresh(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Result<Json<AuthTokens>, Status> {
  if used_token(&client, &refresh_token_data.refresh_token, &config).unwrap() {
    return Err(Status::Unauthorized);
  }
  let user_id = match verify_refresh_token(&refresh_token_data.refresh_token, &config) {
    Some(value) => value,
    None => return Err(Status::Unauthorized),
  };

  save_token(&client, &refresh_token_data.refresh_token, &config);

  let refresh_token = sign_refresh_token(&user_id, &config);
  let access_token = sign_access_token(&user_id, &config); 

  let auth_tokens = AuthTokens {
    refresh_token,
    access_token,
  };

  Ok(Json(auth_tokens))
}

#[post("/logout", data="<refresh_token_data>")]
pub fn logout(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Status {
  save_token(&client, &refresh_token_data.refresh_token, &config);
  Status::Ok
}