#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod database;
mod json;
mod tokens;

use regex::Regex;
use rocket::State;
use rocket_contrib::json::Json;
use bcrypt::{hash, verify};
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};

fn main() {
  rocket::ignite()
    .manage(database::connect())
    .mount("/", routes![index])
    .mount("/api/v1/", routes![refresh, logout])
    .launch();
}

#[get("/")]
fn index() -> &'static str {
  "Welcome to Getty JWT auth server"
}

#[post("/refresh", data="<refresh_token_data>")]
fn refresh(refresh_token_data: Json<json::RefreshTokenData>) -> Json<json::AuthTokens> {
  let user_id = verify_refresh_token(&refresh_token_data.refresh_token).unwrap();

  let refresh_token = sign_refresh_token(&user_id);
  let access_token = sign_access_token(&user_id);

  let auth_tokens = json::AuthTokens { refresh_token, access_token };

  Json(auth_tokens)
}

#[post("/logout", data="<refresh_token_data>")]
fn logout(refresh_token_data: Json<json::RefreshTokenData>) {
  let refresh_token = &refresh_token_data.refresh_token;
}