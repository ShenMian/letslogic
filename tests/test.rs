use letslogic::error::*;

fn get_api_key() -> String {
    dotenv::dotenv().ok();
    std::env::var("API_KEY").expect("failed to get envniroment variable API_KEY")
}

#[tokio::test]
async fn fetch_collections() {
    let collections = letslogic::fetch_collections(&get_api_key()).await.unwrap();
    assert!(!collections.is_empty());
}

#[tokio::test]
async fn fetch_levels() {
    let levels = letslogic::fetch_levels_by_collection_id(&get_api_key(), 1)
        .await
        .unwrap();
    assert!(!levels.is_empty());
}

#[tokio::test]
async fn submit_solution() {
    let api_key = get_api_key();
    assert!(matches!(
        letslogic::submit_solution(&api_key, 1, "R").await,
        Err(SubmitSolutionError::InvalidLevelId)
    ));

    assert!(matches!(
        letslogic::submit_solution(&api_key, 3000, "R").await,
        Err(SubmitSolutionError::InvalidSolution)
    ));

    assert!(
        letslogic::submit_solution(&api_key, 3000, "uuUdrruurrdDLLLrrdLrdrU")
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn get_all_records() {
    let records = letslogic::fetch_all_records(&get_api_key()).await.unwrap();
    assert!(!records.is_empty());
}
