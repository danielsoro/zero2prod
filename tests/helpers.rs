use std::net::TcpListener;

use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub pg_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let configuration =
        zero2prod::configuration::get_configuration().expect("Faile to load the configuration");
    let pg_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");

    let server =
        zero2prod::startup::run(listener, pg_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://localhost:{}", port),
        pg_pool: pg_pool,
    }
}
