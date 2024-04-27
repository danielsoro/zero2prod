mod helpers;

#[tokio::test]
async fn health_check_works() {
    // Arrage
    let address = crate::helpers::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
