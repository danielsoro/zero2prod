use actix_web::{get, HttpResponse};

#[get("")]
pub async fn health() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
