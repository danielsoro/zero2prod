use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pg_pool: PgPool) -> Result<Server, std::io::Error> {
    let pg_pool = web::Data::new(pg_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(pg_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
