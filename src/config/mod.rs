use dotenv;
use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Environment {
    pub endpoint: String,
}

pub fn get_config() -> envy::Result<Environment> {
  dotenv::dotenv().expect("Failed to read .env file");
  envy::from_env::<Environment>()
}