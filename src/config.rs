pub fn load_config() -> Config {
  Config::default()
}

pub struct Config {
  pub secret: String,
  pub refresh_token_exp: u128,
  pub access_token_exp: u128,
  pub database_host: String,
  pub database_namespace: String,
  pub database_set: String,
}

impl Config {
  fn default() -> Config {
    Config {
      secret: "REMEMBER TO CHANGE SECRET VIA ENV VARIABLES!".to_owned(),
      refresh_token_exp: 10 * 24 * 60 * 60 * 1000, // 10 days
      access_token_exp: 10 * 60 * 1000, // 10 minutes
      database_host: "localhost:3000".to_owned(),
      database_namespace: "getty-auth".to_owned(),
      database_set: "tokens".to_string(),
    }
  }
}