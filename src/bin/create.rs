extern crate bible_api;

use std::env;

use dynomite::{
  dynamodb::{
    AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient,
    KeySchemaElement, ProvisionedThroughput,
  },
  retry::Policy,
  Retries,
};
use rusoto_core::Region;

use bible_api::config::get_config;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();

  // Expect Bible Translation Id as input
  if args.len() < 2 {
    panic!("Translation id cannot be empty");
  }

  let table_name = &args[1].to_uppercase();

  println!("Creating Bible Translation table: {}", table_name);

  let environment = match get_config() {
    Ok(environment) => environment,
    Err(_) => panic!("No environment details found"),
  };

  let client = DynamoDbClient::new(Region::Custom {
    name: "us-east-1".into(),
    endpoint: environment.endpoint.into(),
  })
  .with_retries(Policy::default());

  match client
    .create_table(CreateTableInput {
      table_name: table_name.into(),
      key_schema: vec![
        KeySchemaElement {
          attribute_name: "BookIdAndChapter".into(),
          key_type: "HASH".into(),
        },
        KeySchemaElement {
          attribute_name: "Verse".into(),
          key_type: "RANGE".into(),
        },
      ],
      attribute_definitions: vec![
        AttributeDefinition {
          attribute_name: "BookIdAndChapter".into(),
          attribute_type: "S".into(),
        },
        AttributeDefinition {
          attribute_name: "Verse".into(),
          attribute_type: "N".into(),
        },
      ],
      provisioned_throughput: Some(ProvisionedThroughput {
        read_capacity_units: 1,
        write_capacity_units: 1,
      }),
      ..CreateTableInput::default()
    })
    .await
  {
    Ok(output) => println!("Table created successfully: {:#?}", output),
    Err(e) => println!("Failed to create table: {}", e),
  }
}
