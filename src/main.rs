use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

const DEFAULT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 8080);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(DEFAULT_ADDR)?
        // .bind("[::1]:8080")?
        // .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
