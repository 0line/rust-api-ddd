use actix_web::{middleware, web, App, HttpResponse, HttpServer};
extern crate dotenvy;
use crate::shared::infraestruture::router::index::register;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;

struct Server {
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let port = env::var("PORT").unwrap_or("8080".to_string());
        let port = u16::from_str(&port).unwrap();
        Self { port }
    }

    async fn start(&self) -> std::io::Result<()> {
        let mut server = HttpServer::new(|| {
            App::new()
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
                .service(web::scope("/api/v1").configure(register))
                .default_service(web::route().to(HttpResponse::NotFound))
        });
        println!("Listening on port: {}", self.port);
        server = server.bind(("127.0.0.1", self.port))?;
        server.run().await
    }

}

pub async fn run() {
    let server = Server::new();
    server.start().await.unwrap();
}
