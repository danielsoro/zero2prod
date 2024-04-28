mod helpers;

#[tokio::test]
async fn health_check_works() {
    // Arrage
    let address = helpers::spawn_app();
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

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    // Arrage
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_gui%40gmail.com";
    let response = client
        .post(format!("{}/subscribe", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status())
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrage
    let address = helpers::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_gui%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscribe", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
