use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[post("")]
pub async fn subscribes(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> impl Responder {
    log::info!("Saving new subscriber details in the database");
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
    .await
    {
        Ok(_) => {
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!(
                "Failed to execute query when saving new subscriber: {:?}",
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
