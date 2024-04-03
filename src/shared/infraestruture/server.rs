use actix_web::{middleware, web, App, Error, HttpServer};
extern crate dotenv;
use crate::shared::infraestruture::router::index::register;
use dotenv::dotenv;
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

    async fn listen(&self) -> std::io::Result<()> {
        let mut server = HttpServer::new(|| {
            App::new()
                .wrap(middleware::Compress::default())
                .wrap(middleware::Logger::default())
                .service(web::scope("/api/v1").configure(register))
        });
        println!("Listening on port: {}", self.port);
        server = server.bind(("127.0.0.1", self.port))?;
        server.run().await
    }

    /* async fn listen(&mut self) -> std::io::Result<()> {
        self.http_server.bind(("127.0.0.1", self.port))?.run().await
        HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok)))
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }

    fn getHttpServer(&self) -> &HttpServer {
        &self.http_server
    }

    async fn stop(&self) {
        self.http_server.stop(true).await
    } */
}

pub async fn run() {
    let server = Server::new();
    server.listen().await.unwrap();
}
