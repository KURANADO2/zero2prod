use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::io::Error;
use std::net::TcpListener;
use actix_web::web::Form;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    println!("The port is: {}", listener.local_addr()?.port());
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();
    Ok(server)
}