use dynomite::dynamodb::DynamoDbClient;
use serde::Serialize;
use warp::{reject, reply::json, Rejection, Reply};

use crate::model::bible::{find_verses, BibleRow, SearchQuery};

type Result<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Serialize, Eq, PartialEq)]
pub struct SearchResult {
  pub chapter: u16,
  pub verse: u16,
  pub text: String,
  pub translation_id: String,
  pub book_id: String,
  pub book_name: String,
}

impl SearchResult {
  pub fn new(row: &BibleRow) -> SearchResult {
    SearchResult {
      chapter: row.chapter.clone(),
      verse: row.verse.clone(),
      text: row.text.clone(),
      translation_id: row.translation_id.clone(),
      book_id: row.book_id.clone(),
      book_name: row.book_name.clone(),
    }
  }
}

pub async fn search_verses(
  translation_id: String,
  book_id: String,
  chapter: u16,
  query: SearchQuery,
  client: DynamoDbClient,
) -> Result<impl Reply> {
  let verses = find_verses(&client, translation_id, book_id, chapter, &query)
    .await
    .map_err(|e| reject::custom(e))?;

  let results: Vec<SearchResult> = verses.iter().map(|row| SearchResult::new(&row)).collect();
  Ok(json::<Vec<SearchResult>>(&results))
}
