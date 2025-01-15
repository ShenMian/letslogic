use letslogic::*;

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("API_KEY").expect("failed to get envniroment variable API_KEY")
}

#[tokio::test]
async fn test_fetch_collections() {
    let collections = fetch_collections(&get_api_key()).await.unwrap();
    assert!(!collections.is_empty());
}

#[tokio::test]
async fn test_fetch_levels() {
    let levels = fetch_levels_by_collection_id(&get_api_key(), 1)
        .await
        .unwrap();
    assert!(!levels.is_empty());
}

#[tokio::test]
async fn test_submit_solution() {
    let api_key = get_api_key();
    assert!(matches!(
        submit_solution(&api_key, 1, "R").await,
        Err(SubmitSolutionError::InvalidLevelId)
    ));

    assert!(matches!(
        submit_solution(&api_key, 3000, "R").await,
        Err(SubmitSolutionError::InvalidSolution)
    ));

    assert!(submit_solution(&api_key, 3000, "uuUdrruurrdDLLLrrdLrdrU")
        .await
        .is_ok());
}

#[tokio::test]
async fn test_get_all_records() {
    let api_key = get_api_key();
    assert!(fetch_all_records(&api_key).await.is_ok());
}
