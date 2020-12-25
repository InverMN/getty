#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod database;
mod structs;
mod tokens;
mod config;

use rocket::State;
use rocket_contrib::json::Json;
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};
use database::{save_token, used_token};
use rocket::http::Status;
use structs::{AuthTokens, RefreshTokenData};
use aerospike::Client;
use config::{load_config, Config};

fn main() {
  let config = load_config();

  rocket::ignite()
    .manage(database::connect(&config))
    .manage(config)
    .mount("/", routes![index])
    .mount("/api/v1/", routes![refresh, logout])
    .launch();
}

#[get("/")]
fn index() -> &'static str {
  "JWT auth server"
}

#[post("/refresh", data="<refresh_token_data>")]
fn refresh(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Result<Json<AuthTokens>, Status> {
  if used_token(&client, &refresh_token_data.refresh_token, &config).unwrap() {
    return Err(Status::Unauthorized);
  }
  let user_id = verify_refresh_token(&refresh_token_data.refresh_token, &config).unwrap();

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
fn logout(refresh_token_data: Json<RefreshTokenData>, client: State<Client>, config: State<Config>) -> Status {
  save_token(&client, &refresh_token_data.refresh_token, &config);
  Status::Ok
}