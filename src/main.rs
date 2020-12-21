#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use aerospike::{Client, ClientPolicy};

static mut DATABASE_CLIENT: Option<Arc<aerospike::Client>> = None;

fn main() {
  connect_database();
  rocket::ignite().mount("/", routes![index]).launch();
}

fn try_connecting_database_once() {
  let client_policy = ClientPolicy::default();
  let url = "127.0.0.1:3000";
  Client::new(&client_policy, &url).and_then(|i| {
    unsafe {
      Ok(DATABASE_CLIENT = Some(Arc::new(i)))
    }
  }).ok();
}

fn connect_database() {
  try_connecting_database_once();

  unsafe {
    while DATABASE_CLIENT.is_none() {
      println!("Waiting 60 seconds before reconnecting to database...");
      thread::sleep(Duration::from_secs(60));
      try_connecting_database_once();
    }
  }

  println!("Connected to database")
}

#[get("/")]
fn index() -> &'static str {
  "Welcome to Get JWT auth server"
}