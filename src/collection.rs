use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Collection {
    id: i32,
    title: String,
    author: String,
    #[serde(rename = "levels")]
    level_count: i32,
    description: Option<String>,
}

impl Collection {
    pub async fn fetch_levels(&self, api_key: &str) -> Result<Vec<Level>, FetchError> {
        fetch_levels_by_collection_id(api_key, self.id).await
    }
}
