#[test]
fn test_web_smoke() {
    // Verifies web entrypoint dependency linkage and App state initialization
    let state = shared::AppState::new();
    assert_eq!(state.collection.total_count(), 4);
}
