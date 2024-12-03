use std::sync::{Arc};
use crate::scope::users::domain::user_repository::UserRepository;
use crate::scope::users::domain::user::User;
use crate::scope::users::domain::user_email::UserEmail;
use crate::scope::users::domain::user_id::UserId;
use crate::scope::users::domain::user_pwd::UserPwd;
use crate::shared::domain::responder::APIResponse;

#[derive(Clone, Debug)]
pub struct UserRegisterService<R: UserRepository> {
    repository: Arc<R>,
}
#[derive(serde::Deserialize)]
pub struct UserRegisterRequest {
    id: String,
    email: String,
    pwd:String,
    confirmpwd: String
}

impl UserRegisterRequest {
    pub fn new(id: String, email: String, pwd: String, confirmpwd: String) -> Self {
        Self {
            id,
            email,
            pwd,
            confirmpwd
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
        /*let user = User::new(request.id, request.email, request.pwd);
        match user {
            Ok(u) => {
                return self.repository.save(u).await
            }
            Err(error) => APIResponse::new(
                false,
                Some(error.to_string()),
                None,
                None)
        }*/
        /*if let Err(e) = UserId::new(request.id.clone()) {
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
        }*/

        /*let uuid = UserId::new(request.id).unwrap();
        let email = UserEmail::new(request.email).unwrap();
        let pwd = UserPwd::new(request.pwd).unwrap();*/
        let user = User::create(request.id, request.email, request.pwd, request.confirmpwd);
        match user {
            Ok(u) => {
                return self.repository.save(u).await
            }
            Err(..) => APIResponse::new(
                false,
                Some("Ocurrio un error".to_string()),
                None,
                None)
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;
    #[actix_web::test]
    async fn ensure_user_service(){
        let repo = MemoryRepository::new();
        let service = UserRegisterService::new(repo);
        const UUID: &str = "b92f6347-4d73-4427-8ed7-512f9d58738f";
        const EMAIL: &str   = "test@mail.com";
        const PWD: &str  = "Asdf123#";
        const CONFIRMPWD: &str  = "Asdf123#";
        let request = UserRegisterRequest::new(UUID.parse().unwrap(), EMAIL.to_string(), PWD.to_string(), CONFIRMPWD.to_string());
        let result = service.execute(request).await;
        println!("{:?}",  serde_json::to_string(&result));
    }
}