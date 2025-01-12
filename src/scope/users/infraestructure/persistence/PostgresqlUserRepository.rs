use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel};
use crate::scope::users::domain::{user, userEntitySeaOrm};
use crate::scope::users::domain::user::User;
use crate::scope::users::domain::user_repository::UserRepository;
use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::seaorm::seaorm_repository::SeaOrmRepository;
use crate::scope::users::domain::userEntitySeaOrm::Model;

pub struct PostgresqlUserRepository {
    client: Arc<DatabaseConnection>,
}

impl PostgresqlUserRepository {
    pub fn new(client: Arc<DatabaseConnection>) -> Self {
        PostgresqlUserRepository { client }
    }

    pub fn client(&self) -> &DatabaseConnection {
        &self.client
    }
}

#[async_trait]
impl UserRepository for PostgresqlUserRepository {
    async fn save(&self, user: User) -> APIResponse {
        let repository = SeaOrmRepository::new(self.client.clone(), userEntitySeaOrm::Entity);
        let result = repository.persist(userEntitySeaOrm::Model::from(user)).await;
        match result {
            Ok(_) => APIResponse::new(true, Some("User saved successfully".to_string()), None, None),
            Err(e) => APIResponse::new(false, Some("Failed to save user".to_string()), None, Some(vec![e.to_string()])),
        }
    }
}