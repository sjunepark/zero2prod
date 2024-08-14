use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn subscribe(_form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        _form.email,
        _form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to execute query: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
