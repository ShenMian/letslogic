use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Api(String),
}

#[derive(Error, Debug)]
pub enum SubmitSolutionError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("invalid solution")]
    InvalidSolution,
    #[error("invalid level id")]
    InvalidLevelId,
    #[error("{0}")]
    Api(String),
}

#[derive(Error, Debug)]
pub enum FetchRecordsError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}
