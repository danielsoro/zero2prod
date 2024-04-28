use std::net::TcpListener;

use sqlx::PgPool;

pub fn spawn_app(pg_pool: PgPool) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::startup::run(listener, pg_pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://localhost:{}", port)
}
