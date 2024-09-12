use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::{Value};
use crate::scope::users::application::user_register::{UserRegisterRequest, UserRegisterService};
use crate::scope::users::domain::user_repository::UserRepository;
use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;

pub struct RegisterController<R: UserRepository> {
    user_register: UserRegisterService<R>,
}

impl<R:UserRepository> RegisterController<R> {
    pub fn new(user_register: UserRegisterService<R>) -> Self {
        Self {
            user_register
        }
    }
}

impl<R: UserRepository> Controller for RegisterController<R> {
    async fn run(_req: HttpRequest, body: Value) -> HttpResponse {
        let uuid = body.get("uuid").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let email = body.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let pwd = body.get("pwd").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let repository = MemoryRepository::new();
        let user_register = UserRegisterService::new(repository);
        let result = user_register.execute(UserRegisterRequest::new(uuid, email, pwd)).await;
        if !result.get_success(){
            HttpResponse::BadRequest().json(result)
        } else {
            HttpResponse::Created().json(result)
        }

    }
}

/*
#[cfg(test)]
mod tests {
    use actix_web::{test, FromRequest};
use crate::scope::users::domain::user_repository::UserRepository;
use crate::scope::users::application::user_register::UserRegisterService;
use crate::scope::users::infraestructure::controller::register_put_controller::RegisterController;
use serde_json::json;
use actix_web::HttpRequest;
    use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;

    #[actix_web::test]
    async fn ensure_register_put_controller(){
        let controller = RegisterController::new(UserRegisterService::new(MemoryRepository::new()));
        let data = json!({
            "uuid":"b92f6347-4d73-4427-8ed7-512f9d58738f",
            "email":"test@mail.com",
            "pwd": "1234567890"
        });
        let req = test::TestRequest::set_json(&data, data).to_request();
        let resp = controller.run(req, data).await;
        println!("{:?}", resp);
    }
}*/