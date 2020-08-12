use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
  #[error("Invalid query params")]
  InvalidQuery,
  #[error("Failed to query DB")]
  DBError,
}

impl warp::reject::Reject for CustomError {}