use std::net::TcpListener;

use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listemer: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))

    })
    .listen(listemer)?
    .run();

    Ok(server)
}
