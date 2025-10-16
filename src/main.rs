use actix_web::{get, App, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    "ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(health))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
