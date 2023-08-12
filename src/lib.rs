//! lib.rs
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod login;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health-check", web::get().to(health_check))
            .route("/login", web::get().to(login::login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
