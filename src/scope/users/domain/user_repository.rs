
use async_trait::async_trait;
use crate::scope::users::domain::user::User;
use mockall::*;
use crate::scope::users::domain::userEntitySeaOrm::Model;
use crate::shared::domain::responder::APIResponse;

//Repository User
#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: Model) -> APIResponse;
}