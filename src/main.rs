extern crate bible_api;

use dynomite::dynamodb::DynamoDbClient;
use rusoto_core::Region;
use warp::Filter;

use bible_api::config::get_config;
use bible_api::controller::search::search_verses;
use bible_api::middleware::dynamodb_connection::with_dynamo_db;
use bible_api::middleware::error_handler::handle_error;

#[tokio::main]
async fn main() {
    let environment = match get_config() {
        Ok(environment) => environment,
        Err(_) => panic!("No environment details found"),
    };

    let client = DynamoDbClient::new(Region::Custom {
        name: "us-east-1".into(),
        endpoint: environment.endpoint.into(),
    });

    let route = warp::path!(String / String / u16);

    let search = warp::path("search");

    let search_routes = search
        .and(warp::get())
        .and(route)
        .and(warp::query())
        .and(with_dynamo_db(client.clone()))
        .and_then(search_verses);

    let routes = search_routes
        .with(warp::cors().allow_any_origin())
        .recover(handle_error);

    warp::serve(routes).run(([127, 0, 0, 1], 9000)).await;
}
