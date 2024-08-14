use actix_web::{HttpResponse, Responder};

pub async fn subscribe() -> impl Responder {
    HttpResponse::Ok().finish()
}
