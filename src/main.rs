#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod database;
mod structs;
mod tokens;
mod config;
mod routes;

use config::load_config;
use routes::static_rocket_route_info_for_logout;
use routes::static_rocket_route_info_for_refresh;
use routes::static_rocket_route_info_for_index;

fn main() {
  let config = load_config();

  rocket::ignite()
    .manage(database::connect(&config))
    .manage(config)
    .mount("/", routes![index])
    .mount("/api/v1/", routes![refresh, logout])
    .launch();
}