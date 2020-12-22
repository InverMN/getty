extern crate aerospike;

use aerospike::{Client, ClientPolicy};

pub fn connect() -> Client {
  let host = String::from("localhost:3000");
  Client::new(&ClientPolicy::default(), &host).unwrap()
}