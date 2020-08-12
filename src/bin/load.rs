extern crate bible_api;

use dynomite::{
  dynamodb::{DynamoDb, DynamoDbClient, PutItemInput},
  retry::Policy,
  Retries,
};
use rusoto_core::Region;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use bible_api::config::get_config;
use bible_api::model::bible::BibleRow;

#[derive(Serialize, Deserialize, Debug)]
struct RecordRow {
  chapter: u16,
  verse: u16,
  text: String,
  translation_id: String,
  book_id: String,
  book_name: String,
}

impl RecordRow {
  fn convert(&self) -> BibleRow {
    BibleRow {
      book_id_and_chapter: format!("{}#{}", self.book_id, self.chapter),
      chapter: self.chapter.clone(),
      verse: self.verse.clone(),
      text: self.text.clone(),
      translation_id: self.translation_id.clone(),
      book_id: self.book_id.clone(),
      book_name: self.book_name.clone(),
    }
  }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 {
    panic!("Translation Id and File source is required");
  }

  let translation_id = &args[1].to_uppercase();
  let filename = &args[2];

  println!(
    "Processing Bible Translation table: {}, filename: {}",
    translation_id, filename
  );

  let environment = match get_config() {
    Ok(environment) => environment,
    Err(_) => panic!("No environment details found"),
  };

  let client = DynamoDbClient::new(Region::Custom {
    name: "us-east-1".into(),
    endpoint: environment.endpoint.into(),
  })
  .with_retries(Policy::default());

  let mut counter = 0;

  // Read each line from input file
  if let Ok(lines) = read_lines(filename) {
    // Transform and insert to DynamoDb
    for line in lines {
      if let Ok(ip) = line {
        let deserialized: RecordRow = serde_json::from_str(&ip).unwrap();
        let bible_row = deserialized.convert();

        client
          .put_item(PutItemInput {
            table_name: translation_id.clone().into(),
            item: bible_row.clone().into(),
            ..PutItemInput::default()
          })
          .await?;

        counter = counter + 1;

        if counter % 100 == 0 {
          println!("Inserted {} records", counter);
        }
      }
    }
  };

  println!(
    "Loading complete. {} records loaded to {}",
    counter, translation_id
  );

  Ok(())
}
