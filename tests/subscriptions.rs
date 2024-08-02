mod helpers;

#[tokio::test]
async fn subscribe_return_a_200_for_valid_form_data() {
    // Arrage
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.pg_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.name, "le guin");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrage
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing email"),
        ("email=ursula_le_gui%40gmail.com", "missing name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
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

#[tokio::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    // Arrage
    let test_app = helpers::spawn_app().await;
    let client = reqwest::Client::new();
    let big_name = "a".repeat(257).to_string();

    let test_cases = vec![
        (
            format!("name={}&email=ursula_le_gui%40gmail.com", big_name),
            "larg name",
        ),
        (
            "name=%20&email=ursula_le_gui%40gmail.com".to_string(),
            "empty name",
        ),
        (
            "name=/guin&email=ursula_le_gui%40gmail.com".to_string(),
            "forbidden character",
        ),
        (
            "email=ursula_le_gui%40gmail.com".to_string(),
            "without name",
        ),
        ("name=%20&email=%20".to_string(), "both empty"),
        ("name=Francisco&email=%20".to_string(), "email empty"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(format!("{}/subscriptions", test_app.address))
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
