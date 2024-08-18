use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[tracing::instrument(name = "Confirming a pending subscriber.")]
pub async fn confirm(params: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize, Debug)]
pub struct Parameters {
    subscription_token: String,
}
