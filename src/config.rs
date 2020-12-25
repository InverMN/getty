use std::env::var;

pub fn load_config() -> Config {
  Config {
    secret: var("GETTY_SECRET").unwrap_or_else(|_| "REMEMBER TO CHANGE SECRET VIA ENV VARIABLES!".to_string()),
    refresh_token_exp: (var("GETTY_REFRESH_TOKEN_EXP").unwrap_or_else(|_| (10 * 24 * 60 * 60 * 1000).to_string())).parse::<u128>().unwrap(), // 10 days
    access_token_exp: (var("GETTY_ACCESS_TOKEN_EXP").unwrap_or_else(|_| (10 * 60 * 1000).to_string())).parse::<u128>().unwrap(), // 10 minutes
    database_host: var("GETTY_DATABASE_HOST").unwrap_or_else(|_| "localhost:3000".to_string()),
    database_namespace: var("GETTY_DATABASE_NAMESPACE").unwrap_or_else(|_| "getty-auth".to_string()),
    database_set: var("GETTY_DATABASE_SET").unwrap_or_else(|_| "tokens".to_string()),
  }
}

pub struct Config {
  pub secret: String,
  pub refresh_token_exp: u128,
  pub access_token_exp: u128,
  pub database_host: String,
  pub database_namespace: String,
  pub database_set: String,
}