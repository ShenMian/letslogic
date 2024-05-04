mod collection;
mod error;
mod level;

pub use collection::*;
pub use error::*;
pub use level::*;

use serde::{Deserialize, Serialize};
use serde_json as json;

pub async fn fetch_collections(api_key: &str) -> Result<Vec<Collection>, Error> {
    let url = format!("https://letslogic.com/api/v1/collections?key={api_key}");
    let response = reqwest::get(url).await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(Error::Other(msg.to_string()));
    }

    let collections = json::from_value(json).unwrap();
    Ok(collections)
}

pub async fn fetch_levels_by_collection_id(
    api_key: &str,
    collection_id: i32,
) -> Result<Vec<Level>, Error> {
    let url = format!("https://letslogic.com/api/v1/collection/{collection_id}?key={api_key}");
    let response = reqwest::get(url).await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return Err(Error::Other(msg.to_string()));
    }

    let levels = json::from_value(json).unwrap();
    Ok(levels)
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SubmitResult {
    result: String,
    blue: SubmitResultDetails,
    green: SubmitResultDetails,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SubmitResultDetails {
    rank: i32,
    points: i32,
    moves: i32,
    pushes: i32,
}

pub async fn submit_solution(
    api_key: &str,
    level_id: i32,
    solution: &str,
) -> Result<SubmitResult, Error> {
    let url =
        format!("https://letslogic.com/api/v1/level/{level_id}?key={api_key}&solution={solution}");
    let response = reqwest::get(url).await?;
    let json: json::Value = json::from_slice(&response.bytes().await?)?;

    if let Some(msg) = json.get("error") {
        return match msg.as_str().unwrap() {
            "Specified level does not exist" => Err(SubmitSolutionError::InvalidLevelId.into()),
            "Invalid Solution" => Err(SubmitSolutionError::InvalidSolution.into()),
            _ => Err(SubmitSolutionError::Other(msg.to_string()).into()),
        };
    }

    let result = json::from_value(json).unwrap();
    Ok(result)
}
