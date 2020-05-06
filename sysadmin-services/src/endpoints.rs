use actix_web::{get,Responder};

#[get("/")]
async fn index() -> impl Responder {
    format!("Endpoints: /test")
}