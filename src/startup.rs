use crate::routes::health;
use crate::routes::subscribe;
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_pool = web::Data::new(pg_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health)
            .service(subscribe)
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
