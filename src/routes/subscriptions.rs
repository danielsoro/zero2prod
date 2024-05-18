use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { name, email })
    }
}

#[post("")]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pg_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribes(form: web::Form<FormData>, pg_pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = match form.into_inner().try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pg_pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pg_pool, new_subscriber)
)]
pub async fn insert_subscriber(
    pg_pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pg_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
