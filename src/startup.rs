use crate::routes::health;
use crate::routes::subscribes;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_pool = web::Data::new(pg_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(web::scope("health").service(health))
            .service(web::scope("subscriptions").service(subscribes))
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
