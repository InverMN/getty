#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod database;
mod json;
mod tokens;

use rocket::State;
use rocket_contrib::json::Json;
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};
use database::{save_token, used_token};
use rocket::http::Status;
use json::AuthTokens;

fn main() {
  rocket::ignite()
    .manage(database::connect())
    .mount("/", routes![index])
    .mount("/api/v1/", routes![refresh, logout])
    .launch();
}

#[get("/")]
fn index() -> &'static str {
  "JWT auth server"
}

#[post("/refresh", data="<refresh_token_data>")]
fn refresh(refresh_token_data: Json<json::RefreshTokenData>, client: State<aerospike::Client>) -> Result<Json<json::AuthTokens>, Status> {
  if used_token(&client, &refresh_token_data.refresh_token).unwrap() {
    return Err(Status::Unauthorized);
  }
  let user_id = verify_refresh_token(&refresh_token_data.refresh_token).unwrap();

  save_token(&client, &refresh_token_data.refresh_token);

  let refresh_token = sign_refresh_token(&user_id);
  let access_token = sign_access_token(&user_id); 

  let auth_tokens = AuthTokens {
    refresh_token,
    access_token,
  };

  Ok(Json(auth_tokens))
}

#[post("/logout", data="<refresh_token_data>")]
fn logout(refresh_token_data: Json<json::RefreshTokenData>, client: State<aerospike::Client>) {
  save_token(&client, &refresh_token_data.refresh_token);
}