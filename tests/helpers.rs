use std::net::TcpListener;

use once_cell::sync::Lazy;
use sqlx::{Executor, PgPool};
use uuid::Uuid;
use zero2prod::{
    configuration::{get_configuration, DatabaseSettings},
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}

async fn create_test_database(config: &DatabaseSettings) {
    let connection = PgPool::connect_with(config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    create_test_database(config).await;

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Faile to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let pg_pool = configure_database(&configuration.database).await;

    let server = run(listener, pg_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{}", port),
        pg_pool,
    }
}
