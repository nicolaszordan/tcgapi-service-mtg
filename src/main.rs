use actix_web::{get, middleware::Logger, App, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    "ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(health))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
