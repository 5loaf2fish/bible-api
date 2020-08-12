use dynomite::dynamodb::DynamoDbClient;
use std::convert::Infallible;
use warp::Filter;

pub fn with_dynamo_db(db_client: DynamoDbClient) -> impl Filter<Extract = (DynamoDbClient,), Error = Infallible> + Clone {
  warp::any().map(move || db_client.clone())
}