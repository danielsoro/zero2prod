use sqlx::{PgPool};
use std::net::TcpListener;

use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscribe, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscribe("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuraion");

    let pg_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to the database");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, pg_pool)?.await
}
