mod collection;
pub mod error;
mod level;

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use serde::{Deserialize, Serialize};
use serde_json as json;
use zip::ZipArchive;

pub use collection::*;
use error::*;
pub use level::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SubmitSolutionResult {
    result: String,
    #[serde(rename = "blue")]
    best_move: Record,
    #[serde(rename = "green")]
    best_push: Record,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct LevelRecord {
    #[serde(rename = "blue")]
    best_move: Record,
    #[serde(rename = "green")]
    best_push: Record,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Record {
    rank: i32,
    points: i32,
    moves: i32,
    pushes: i32,
}

/// Let's Logic API: Get collection list.
pub async fn fetch_collections(api_key: &str) -> Result<Vec<Collection>, FetchError> {
    let url = "https://letslogic.com/api/v1/collections";
    let response = reqwest::Client::new()
        .post(url)
        .form(&[("key", api_key)])
        .send()
        .await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(FetchError::Api(msg.to_string()));
    }

    Ok(json::from_value(json)?)
}

/// Let's Logic API: Get levels in collection.
pub async fn fetch_levels_by_collection_id(
    api_key: &str,
    collection_id: i32,
) -> Result<Vec<Level>, FetchError> {
    let url = format!("https://letslogic.com/api/v1/collection/{collection_id}");
    let response = reqwest::Client::new()
        .post(url)
        .form(&[("key", api_key)])
        .send()
        .await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(FetchError::Api(msg.to_string()));
    }

    Ok(json::from_value(json)?)
}

/// Let's Logic API: Submit level solution.
pub async fn submit_solution(
    api_key: &str,
    level_id: i32,
    solution: &str,
) -> Result<SubmitSolutionResult, SubmitSolutionError> {
    let url = format!("https://letslogic.com/api/v1/level/{level_id}");
    let response = reqwest::Client::new()
        .post(url)
        .form(&[("key", api_key), ("solution", solution)])
        .send()
        .await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return match msg.as_str().unwrap() {
            "Specified level does not exist" => Err(SubmitSolutionError::InvalidLevelId),
            "Invalid Solution" => Err(SubmitSolutionError::InvalidSolution),
            _ => Err(SubmitSolutionError::Api(msg.to_string())),
        };
    }

    Ok(json::from_value(json)?)
}

/// Let's Logic API: Get records for completed levels.
pub async fn fetch_all_records(
    api_key: &str,
) -> Result<HashMap<i32, LevelRecord>, FetchRecordsError> {
    let url = "https://letslogic.com/api/v1/records";
    let response = reqwest::Client::new()
        .post(url)
        .form(&[("key", api_key)])
        .send()
        .await?;

    let content = Cursor::new(response.bytes().await?);
    let mut archive = ZipArchive::new(content)?;
    let mut json_file = archive.by_index(0)?;

    let mut buf = String::new();
    json_file
        .read_to_string(&mut buf)
        .expect("failed to read file");

    Ok(json::from_str(&buf)?)
}
