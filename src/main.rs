#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod metrics;
mod database;
mod structs;
mod tokens;
mod config;
mod routes;

use config::load_config;
use metrics::register_metrics;
use routes::static_rocket_route_info_for_logout;
use routes::static_rocket_route_info_for_refresh;
use routes::static_rocket_route_info_for_index;
use routes::static_rocket_route_info_for_metrics;
use routes::static_rocket_route_info_for_get_token;

fn main() {
  register_metrics();
  let config = load_config();

  rocket::ignite()
    .manage(database::connect(&config))
    .manage(config)
    .mount("/", routes![index, metrics, get_token])
    .mount("/api/v1/", routes![refresh, logout])
    .launch();
}