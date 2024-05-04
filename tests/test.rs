#[cfg(test)]
mod tests {

    use letslogic::*;

    /// WARNING: Exposing this API key does not cause serious security issues
    const API_KEY: &str = "455f618bab67eee03fc35f32a984b57d8afc789c45527e1820e365acac445623";

    #[tokio::test]
    async fn test_fetch_collections() {
        let collections = fetch_collections(API_KEY).await.unwrap();
        assert!(!collections.is_empty());
    }

    #[tokio::test]
    async fn test_fetch_levels() {
        let levels = fetch_levels_by_collection_id(API_KEY, 1).await.unwrap();
        assert!(!levels.is_empty());
    }

    #[tokio::test]
    async fn test_submit_solution() {
        assert!(matches!(
            submit_solution(API_KEY, 1, "R").await,
            Err(Error::SubmitSolution(SubmitSolutionError::InvalidLevelId))
        ));

        assert!(matches!(
            submit_solution(API_KEY, 3000, "R").await,
            Err(Error::SubmitSolution(SubmitSolutionError::InvalidSolution))
        ));

        assert!(submit_solution(API_KEY, 3000, "uuUdrruurrdDLLLrrdLrdrU")
            .await
            .is_ok());
    }
}
