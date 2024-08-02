use actix_web::{
    post,
    web::{self, Form},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};

#[derive(serde::Deserialize)]
pub struct SubscriberFormData {
    name: String,
    email: String,
}

impl TryInto<NewSubscriber> for Form<SubscriberFormData> {
    type Error = String;
    fn try_into(self) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(self.0.name)?;
        let email = SubscriberEmail::parse(self.0.email)?;
        Ok(NewSubscriber { name, email })
    }
}

impl TryFrom<SubscriberFormData> for NewSubscriber {
    type Error = String;
    fn try_from(value: SubscriberFormData) -> Result<NewSubscriber, Self::Error> {
        value.try_into()
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
pub async fn subscribes(
    form: web::Form<SubscriberFormData>,
    pg_pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_subscriber = match form.try_into() {
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
