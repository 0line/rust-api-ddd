use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use dotenvy::dotenv;
use std::env;
use crate::shared::infraestruture::seaorm::seaorm_config::SeaOrmConfig;

pub enum DatabaseClient {
    Postgres,
    Sqlite,
    Default,
}

pub struct SeaOrmClientFactory {
    client: Option<DatabaseConnection>,
}

impl SeaOrmClientFactory {
    pub async fn new(config: SeaOrmConfig, database: DatabaseClient) -> Result<DatabaseConnection, DbErr> {
        let url_db = match database {
            DatabaseClient::Postgres => format!(
                "postgres://{}:{}@{}:{}/{}?currentSchema={}",
                config.username,
                config.password,
                config.host,
                config.port,
                config.database,
                config.schema.unwrap_or("public".to_string())
            ),
            DatabaseClient::Sqlite => format!("sqlite://{}", config.database),
            DatabaseClient::Default => "sqlite::memory:".to_string(),
        };

        let mut optconnection = ConnectOptions::new(&url_db);
        optconnection
            .max_connections(3)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(60))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true)
            .sqlx_logging_level(log::LevelFilter::Info);

        match Database::connect(optconnection).await {
            Ok(dbconnection) => Ok(dbconnection),
            Err(e) => {
                eprintln!("Failed to connect to the database: {:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::shared::infraestruture::seaorm::seaorm_config::SeaOrmConfig;
    use crate::shared::infraestruture::seaorm::seaorm_client_factory::DatabaseClient;
    use sea_orm::{Database, DatabaseConnection, EntityTrait, IntoActiveModel, ActiveModelTrait};
    use crate::scope::users::domain::user::User;
    use crate::scope::users::domain::user_repository::UserRepository;

    #[actix_rt::test]
    async fn test_connect() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Postgres).await;
        assert!(db.is_ok());
    }

    #[actix_rt::test]
    async fn test_connect_sqlite() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers.db".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Sqlite).await;
        assert!(db.is_ok());
    }

    #[actix_rt::test]
    async fn test_connect_memory() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers.db".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Default).await;
        assert!(db.is_ok());
    }

    #[actix_rt::test]
    async fn test_connect_fail() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Postgres).await;
        assert!(db.is_err());
    }

    #[actix_rt::test]
    async fn test_connect_fail_memory() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers.db".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Default).await;
        assert!(db.is_err());
    }

    #[actix_rt::test]
    async fn test_connect_fail_sqlite() {
        let config = SeaOrmConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "0line".to_string(),
            password: "0linetest".to_string(),
            database: "apiusers.db".to_string(),
            schema: Some("public".to_string()),
        };
        let db = SeaOrmClientFactory::new(config, DatabaseClient::Sqlite).await;
        assert!(db.is_err());
    }
}