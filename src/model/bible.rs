use dynomite::Item;
use serde::Deserialize;
use dynomite::{
  attr_map,
  dynamodb::{DynamoDbClient, QueryInput},
  DynamoDbExt, FromAttributes,
};
use futures::{future, TryStreamExt};

use crate::types::error::CustomError;

#[derive(Item, Debug, Clone)]
pub struct BibleRow {
  #[dynomite(partition_key, rename = "BookIdAndChapter")]
  pub book_id_and_chapter: String,
  #[dynomite(rename = "Chapter", default)]
  pub chapter: u16,
  #[dynomite(rename = "Verse", default)]
  pub verse: u16,
  #[dynomite(rename = "Text", default)]
  pub text: String,
  #[dynomite(rename = "TranslationId", default)]
  pub translation_id: String,
  #[dynomite(rename = "BookId", default)]
  pub book_id: String,
  #[dynomite(rename = "BookName", default)]
  pub book_name: String,
}

#[derive(Deserialize, Debug)]
pub struct SearchQuery {
    from: Option<u16>,
    to: Option<u16>,
}

pub async fn find_verses(
  client: &DynamoDbClient,
  translation_id: String,
  book_id: String,
  chapter: u16,
  query: &SearchQuery,
) -> Result<Vec<BibleRow>, CustomError> {
  let mut results = vec![];

  let verse_start = match query.from {
    Some(from) => from,
    _ => 0,
  };

  let verse_end = match query.to {
    Some(to) => to,
    _ => 999,
  };

  let table_name = translation_id.to_uppercase();

  client
    .clone()
    .query_pages(QueryInput {
      table_name: table_name.clone().into(),
      key_condition_expression: Some(
        "BookIdAndChapter = :bookChapter AND Verse BETWEEN :verse_start AND :verse_end".into(),
      ),
      expression_attribute_values: Some(attr_map!(
          ":bookChapter" => format!("{}#{}",book_id,chapter),
          ":verse_start" => verse_start,
          ":verse_end" => verse_end,
      )),
      ..QueryInput::default()
    })
    .try_for_each(|item| {
      results.push(BibleRow::from_attrs(item).unwrap());
      future::ready(Ok(()))
    })
    .await.map_err(|e| { println!("DB Error: {:?}", e); CustomError::DBError })?;

  Ok(results)
}
