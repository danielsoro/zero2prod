use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::{info_span, Instrument};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[post("")]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pg_pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribes(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> impl Responder {
    let query_span = info_span!("Saving new subscriber details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pg_pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query when saving new subscriber {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
