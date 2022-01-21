use actix_web::{App, HttpServer};
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(service::login)
            .service(service::temp_token)
            .service(service::access_token)
    })
    .bind("127.0.0.1:3333")?
    .run()
    .await
}
