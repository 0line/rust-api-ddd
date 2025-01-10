use actix_web::{middleware, web, App, HttpResponse, HttpServer};
extern crate dotenvy;
use crate::shared::infraestruture::router::index::register;
use dotenvy::dotenv;
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::shared::infraestruture::seaorm::seaorm_client_factory::DatabaseClient::Postgres;
use crate::shared::infraestruture::seaorm::seaorm_client_factory::SeaOrmClientFactory;

struct Server {
   port: u16,
}

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

impl Server {
   pub fn new() -> Self {
        dotenv().ok();
        let port = env::var("PORT").unwrap_or("8080".to_string());
        let port = u16::from_str(&port).unwrap();
        Self { port }
    }

    async fn start(&self) -> std::io::Result<()>
    {
        dotenv().ok();
        let port = env::var("PORT").unwrap_or("8080".to_string());
        let port = u16::from_str(&port).unwrap();

        let config  = crate::shared::infraestruture::seaorm::seaorm_config::SeaOrmConfig {
            host: env::var("DB_HOST").unwrap_or("0.0.0.0".to_string()),
            port: env::var("DB_PORT").unwrap_or("5432".to_string()).parse().unwrap(),
            username: env::var("DB_USER").unwrap_or("".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or("".to_string()),
            database: env::var("DB_NAME").unwrap_or("".to_string()),
            schema: Some(env::var("DB_SCHEMA").unwrap_or("public".to_string())),
        };

        let db_connection = SeaOrmClientFactory::new(config, Postgres).await.unwrap();
        let db_connection = Arc::new(db_connection);
        let mut server =HttpServer::new(move|| {
            App::new()
                .wrap(middleware::Logger::default())
                .app_data(actix_web::web::Data::new(AppState { db: db_connection.clone() }))
                .service(web::scope("/api/v1").configure(register))
                .default_service(web::route().to(HttpResponse::NotFound))
                .configure(register)
        });
        println!("Listening on port: {}", port.clone());
        server = server.bind(("127.0.0.1", port.clone()))?;
        server.run().await

    }

}

pub async fn run() {
    let server = Server::new();
    server.start().await.unwrap();
}
