use std::sync::{Arc, Mutex, RwLock};
use actix_web::Error;
use crate::scope::users::domain::user_repository::UserRepository;
use crate::scope::users::domain::user::User;
use crate::scope::users::domain::user_email::UserEmail;
use crate::scope::users::domain::user_id::UserId;
use crate::scope::users::domain::user_pwd::UserPwd;
use crate::shared::domain::responder;
use crate::shared::domain::responder::APIResponse;

#[derive(Clone, Debug)]
pub struct UserRegisterService<R: UserRepository> {
    repository: Arc<R>,
}
#[derive(serde::Deserialize)]
pub struct UserRegisterRequest {
    id: String,
    email: String,
    pwd:String
}

impl UserRegisterRequest {
    pub fn new(id: String, email: String, pwd: String) -> Self {
        Self {
            id,
            email,
            pwd
        }
    }
}
impl<R: UserRepository> UserRegisterService<R> {
    pub fn new(repository: R) -> Self {
        UserRegisterService {
            repository: Arc::new(repository),
        }
    }

    pub async fn execute(&self, request:UserRegisterRequest) -> APIResponse {
        let mut error_vec: Vec<String> = Vec::new();
        if let Err(e) = UserId::new(request.id.clone()) {
            error_vec.push(e.to_string());
        }

        if let Err(e) = UserEmail::new(request.email.clone()) {
            error_vec.push(e.to_string());
        }

        if let Err(e) = UserPwd::new(request.pwd.clone()) {
            error_vec.push(e.to_string());
        }

        if !error_vec.is_empty() {
            return APIResponse::new(
                false,
                Some("Ocurrio un error".to_string()),
                None,
                Some(error_vec));
        }

        let uuid = UserId::new(request.id).unwrap();
        let email = UserEmail::new(request.email).unwrap();
        let pwd = UserPwd::new(request.pwd).unwrap();
        let user = User::new(uuid, email, pwd);
        let mut error_vec: Vec<String> = Vec::new();
        match user {
            Ok(u) => {
                self.repository.save(u).await
            }
            Err(e) => {
                // Manejar el error aquí, por ejemplo, devolviendo un mensaje de error
                APIResponse::new(
                    false,
                    Some("Ocurrio un error".to_string()),
                    None,
                    None)
            }
        }
        /*let user = User::new(uuid, email, pwd);
        let mut error_vec: Vec<String> = Vec::new();
        if let Err(e) = self.repository.save(user){
            error_vec.push(e.to_string());
        }
        if !error_vec.is_empty() {
            APIResponse::new(
                false,
                Some("Ocurrio un error".to_string()),
                None,
                Some(error_vec));
        }

*/
    }
}
#[cfg(test)]
mod tests {
    use std::process::id;
    use super::*;
    use mockall::predicate::*;
    use mockall::*;
    use serde_json::json;
    use crate::scope::users::domain::user_repository::{UserRepository, InMemoryUserRepository};

    #[actix_web::test]
    async fn ensure_user_service(){
       //let mut repo = UserRepository::new();
        let repo = InMemoryUserRepository::new();
        let service = UserRegisterService::new(repo);
        const UUID: &str = "b92f6347-4d73-4427-8ed7-512f9d58738f";
        const EMAIL: &str   = "test@mail.com";
        const PWD: &str  = "Asdf123#";
        let request = UserRegisterRequest::new(
            UUID.parse().unwrap(),
            EMAIL.to_string(),
            PWD.to_string()
        );
        let result = service.execute(request).await;
        println!("{:?}",  serde_json::to_string(&result));

   }
}