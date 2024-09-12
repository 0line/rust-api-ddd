
use async_trait::async_trait;
use crate::scope::users::domain::user::User;
use mockall::*;
use crate::shared::domain::responder::APIResponse;

#[automock]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> APIResponse;
}

/*pub struct InMemoryUserRepository {
    users: RwLock<Vec<User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository {
            users: RwLock::new(Vec::new()),
        }
    }
}
#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> APIResponse {
        let mut error_vec: Vec<String> = Vec::new();
        if let Err(e) = self.users.write() {
            error_vec.push(e.to_string());
        }
        if !error_vec.is_empty() {
            return APIResponse::new(
                false,
                Some("Ocurrio un error".to_string()),
                None,
                Some(error_vec));
        }
        let mut users = self.users.write().unwrap();
        users.push(user.clone());
        let concatenated = format!("Se creó correctamente el usuario {}", user.get_email());
        APIResponse::new(
            true,
            Some(concatenated.to_string()),
            None,
            None)
    }
}*/