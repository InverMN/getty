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
    match Client::new(&ClientPolicy::default(), &host) {
      Ok(value) => {
        client = Some(value);
        break;
      },
      Err(_) => {
        if first_connect_try {
          first_connect_try = false;
          println!("Failed to connect to Aerospike database at host \"{}\", retrying continuously...", &host);
        }
      },
    }
    thread::sleep(five_seconds);
  }

  println!("Connected do database successfully");

  client.unwrap()
}

pub fn save_token(client: &Client, token: &str, config: &Config) {
  let policy = WritePolicy { expiration: Expiration::Seconds((config.refresh_token_exp / 1000 + 1) as u32), ..Default::default() };

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