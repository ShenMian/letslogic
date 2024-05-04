use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    SubmitSolution(#[from] SubmitSolutionError),
    #[error("{0}")]
    Other(String),
}

#[derive(Error, Eq, PartialEq, Debug)]
pub enum SubmitSolutionError {
    #[error("invalid solution")]
    InvalidSolution,
    #[error("invalid level id")]
    InvalidLevelId,
    #[error("{0}")]
    Other(String),
}
