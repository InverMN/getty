use mongodb::{
  bson::{doc, Bson},
  sync::{Client, Collection},
};

pub fn connect() -> Client {
  Client::with_uri_str("mongodb://localhost:27017").unwrap()
}

pub fn user_collection(client: &Client) -> Collection {
  let database = client.database("database");
  database.collection("users")
}