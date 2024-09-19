
use async_trait::async_trait;
use crate::scope::users::domain::user::User;
use mockall::*;
use crate::shared::domain::responder::APIResponse;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> APIResponse;
}