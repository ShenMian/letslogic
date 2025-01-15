mod collection;
mod error;
mod level;

use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use serde::{Deserialize, Serialize};
use serde_json as json;
use zip::ZipArchive;

pub use collection::*;
pub use error::*;
pub use level::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SubmitResult {
    result: String,
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct LevelRecord {
    #[serde(rename = "blue")]
    best_move: Record,
    #[serde(rename = "green")]
    best_push: Record,
}

pub async fn fetch_collections(api_key: &str) -> Result<Vec<Collection>, Error> {
    let url = "https://letslogic.com/api/v1/collections";
    let client = reqwest::Client::new();
    let response = client.post(url).form(&[("key", api_key)]).send().await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(Error::Other(msg.to_string()));
    }

    let collections = json::from_value(json)?;
    Ok(collections)
}

pub async fn fetch_levels_by_collection_id(
    api_key: &str,
    collection_id: i32,
) -> Result<Vec<Level>, Error> {
    let url = format!("https://letslogic.com/api/v1/collection/{collection_id}");
    let client = reqwest::Client::new();
    let response = client.post(url).form(&[("key", api_key)]).send().await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(Error::Other(msg.to_string()));
    }

    let levels = json::from_value(json)?;
    Ok(levels)
}

pub async fn submit_solution(
    api_key: &str,
    level_id: i32,
    solution: &str,
) -> Result<SubmitResult, Error> {
    let url = format!("https://letslogic.com/api/v1/level/{level_id}");
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .form(&[("key", api_key), ("solution", solution)])
        .send()
        .await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return match msg.as_str().unwrap() {
            "Specified level does not exist" => Err(SubmitSolutionError::InvalidLevelId.into()),
            "Invalid Solution" => Err(SubmitSolutionError::InvalidSolution.into()),
            _ => Err(SubmitSolutionError::Other(msg.to_string()).into()),
        };
    }

    let result = json::from_value(json)?;
    Ok(result)
}

pub async fn get_all_records(api_key: &str) -> Result<HashMap<i32, LevelRecord>, Error> {
    let url = "https://letslogic.com/api/v1/records";
    let client = reqwest::Client::new();
    let response = client.post(url).form(&[("key", api_key)]).send().await?;

    let content = Cursor::new(response.bytes().await?);
    let mut archive = ZipArchive::new(content).expect("failed to open zip archive");
    let mut json_file = archive
        .by_index(0)
        .expect("failed to get file from zip archive");

    let mut buf = String::new();
    json_file.read_to_string(&mut buf).unwrap();
    dbg!(&buf);

    let records = json::from_str(&buf)?;
    Ok(records)
}
