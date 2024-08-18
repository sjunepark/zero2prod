use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use uuid::Uuid;

use crate::domain::NewSubscriber;
use crate::email_client::EmailClient;
use crate::startup::ApplicationBaseUrl;

#[instrument(
    name = "Adding a new subscriber.",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> impl Responder {
    let new_subscriber = match form.0.try_into() {
        Ok(new_subscriber) => new_subscriber,
        Err(_) => {
            error!("Failed to parse the form data.");
            return HttpResponse::BadRequest().finish();
        }
    };

    if let Err(e) = insert_subscriber(&new_subscriber, &pool).await {
        error!(
            error.cause_chain = ?e,
            error.message = %e,
            "Failed to store new subscriber in the database."
        );
        return HttpResponse::InternalServerError().finish();
    }

    if let Err(e) = send_confirmation_email(&email_client, &new_subscriber, &base_url).await {
        error!(
            error.cause_chain = ?e,
            error.message = %e,
            "Failed to send a confirmation email.");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[instrument(
    name = "Sending a confirmation email to a new subscriber.",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: &NewSubscriber,
    base_url: &ApplicationBaseUrl,
) -> Result<(), reqwest::Error> {
    let confirmation_link = format!(
        "{}/subscriptions/confirm?subscription_token=mytoken",
        base_url.0
    );
    let plain_body = format!(
        "Welcome to the newsletter, {}! Click here to confirm your subscription: {}",
        new_subscriber.name, confirmation_link
    );
    let html_body = format!(
        "Welcome to the newsletter, {}! Click <a href=\"{}\">here</a> to confirm your subscription.",
        new_subscriber.name, confirmation_link
    );

    email_client
        .send_email(&new_subscriber.email, "Welcome!", &html_body, &plain_body)
        .await
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
    INSERT INTO subscriptions (id, email, name, subscribed_at, status)
    VALUES ($1, $2, $3, $4, 'pending_confirmation')
    "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map(|result| {
        info!("New subscriber details have been saved.");
        result
    })
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
