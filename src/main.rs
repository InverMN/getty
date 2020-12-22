#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod aerospike_database;
mod mongo_database;
mod json;
mod tokens;

use regex::Regex;
use rocket::State;
use rocket_contrib::json::Json;
use mongodb::{
  bson::{doc, Bson},
  sync::{Client, Collection},
};
use bcrypt::{hash, verify};
use tokens::{sign_refresh_token, sign_access_token, verify_refresh_token};

fn main() {
  let aerospike_client = aerospike_database::connect();
  let mongo_client = mongo_database::connect();

  rocket::ignite()
    .manage(mongo_client)
    .manage(aerospike_client)
    .mount("/", routes![index])
    .mount("/api/v1/", routes![register, login, refresh, logout])
    .launch();
}

#[get("/")]
fn index() -> &'static str {
  "Welcome to Getty JWT auth server"
}

#[post("/register", data = "<register_data>")]
fn register(register_data: Json<json::RegisterData>, mongo_client: State<Client>) -> Json<json::AuthTokens> {
  let user_collection = mongo_database::user_collection(&mongo_client);

  let hashed_password = hash(&register_data.password, 8).unwrap();

  let register_data_document = doc! {
    "email": &register_data.email,
    "username": &register_data.username,
    "hashedPassword": hashed_password,
  };

  let insertion_result = user_collection.insert_one(register_data_document, None).unwrap();
  let user_id_object = insertion_result.inserted_id.as_object_id().unwrap();
  let user_id = user_id_object.to_hex();

  let refresh_token = sign_refresh_token(&user_id);
  let access_token = sign_access_token(&user_id);

  let auth_tokens = json::AuthTokens { refresh_token, access_token };

  Json(auth_tokens)
}

#[post("/login", data = "<login_data>")]
fn login(login_data: Json<json::LoginData>, mongo_client: State<Client>) -> Json<json::AuthTokens> {
  let user_collection = mongo_database::user_collection(&mongo_client);

  let search_document = doc! {
    "email": &login_data.email
  };

  let user_document = user_collection.find_one(Some(search_document), None).unwrap().unwrap();
  let hashed_password = user_document.get_str("hashedPassword").unwrap();
  
  if verify(&login_data.password, hashed_password).unwrap() {
    println!("Passwords are the same");
  } else {
    println!("Passwords are different");
  }

  let user_id_object = user_document.get_object_id("_id").unwrap();
  let user_id = user_id_object.to_hex();
  
  let refresh_token = sign_refresh_token(&user_id);
  let access_token = sign_access_token(&user_id);

  let auth_tokens = json::AuthTokens { refresh_token, access_token };

  Json(auth_tokens)
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