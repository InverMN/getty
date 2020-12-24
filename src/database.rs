extern crate aerospike;

use crate::config;

use aerospike::{Client, ClientPolicy, as_bin, as_key, Expiration, WritePolicy};
use std::{thread, time};
use time::{Duration};
use config::Config;

pub fn connect(config: &Config) -> Client {
  let host = String::from(&config.database_host);
  let client: Option<Client>;

  let mut first_connect_try = true;
  let five_seconds = Duration::from_secs(5);

  loop {
    let connection_result = Client::new(&ClientPolicy::default(), &host);

    if connection_result.is_ok() {
      client = Some(connection_result.unwrap());
      break;
    }

    if first_connect_try {
      first_connect_try = false;
      println!("Failed to connect to Aerospike database at host \"{}\", retrying continuously...", &host);
    }

    thread::sleep(five_seconds);
  }

  println!("Connected do database successfully");

  client.unwrap()
}

pub fn save_token(client: &Client, token: &str, config: &Config) {
  let mut policy = WritePolicy::default();
  policy.expiration = Expiration::Seconds(60 * 60 * 24 * 11);

  let key = as_key!(&config.database_namespace, &config.database_set, token);
  let bins = vec![
    as_bin!("a", ""),
  ];

  let writing_result =  client.put(&policy, &key, &bins);
  if writing_result.is_err() {
    panic!("Error writing refresh token to database... \n{}", writing_result.unwrap_err());
  }
}

pub fn used_token(client: &Client, token: &str, config: &Config) -> aerospike::errors::Result<bool> {
  let policy = WritePolicy::default();
  let key = as_key!(&config.database_namespace, &config.database_set, token);
  client.exists(&policy, &key)
}