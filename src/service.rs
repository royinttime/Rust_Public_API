use actix_web::{get, post, HttpResponse, Responder};

#[get("/api/connect")]
pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/api/connect/authorize")]
pub async fn temp_token(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/api/access_token")]
pub async fn access_token(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
