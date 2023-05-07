use actix_web::{HttpResponse, Responder};

pub const PORT: u16 = 8080;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server  is running!")
}
