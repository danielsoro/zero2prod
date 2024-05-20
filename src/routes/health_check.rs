use actix_web::{get, web, HttpResponse};
use sqlx::{pool::PoolConnection, PgPool, Postgres};

#[get("")]
pub async fn health(pg_pool: web::Data<PgPool>) -> HttpResponse {
    match health_db(&pg_pool).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn health_db(pg_pool: &PgPool) -> Result<PoolConnection<Postgres>, sqlx::Error> {
    pg_pool.acquire().await.map_err(|e| {
        tracing::error!("Not able to get connection with database: {}", e);
        e
    })
}
