use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::NewSubscriber;

#[instrument(
    name = "Adding a new subscriber.",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let new_subscriber = match form.0.try_into() {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&new_subscriber, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[instrument(
    name = "Saving new subscriber details in the database.",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    pool: &PgPool,
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
    .execute(pool)
    .await
    .map(|result| {
        tracing::info!("New subscriber details have been saved.");
        result
    })
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
