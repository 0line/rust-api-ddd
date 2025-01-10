use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::{Value};
use crate::scope::users::application::user_register::{UserRegisterRequest, UserRegisterService};
use crate::scope::users::domain::user_repository::UserRepository;
use crate::scope::users::infraestructure::persistence::PostgresqlUserRepository::PostgresqlUserRepository;
use crate::shared::infraestruture::server::AppState;
//use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;

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
    async fn run(_req: HttpRequest, body: Value, data: web::Data<AppState>) -> HttpResponse {
        let uuid = body.get("uuid").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let email = body.get("email").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let pwd = body.get("pwd").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let confirm_pwd = body.get("confirm_pwd").and_then(|v| v.as_str()).unwrap_or("").to_string();
        let repository = PostgresqlUserRepository::new(data.db.clone());
        let user_register = UserRegisterService::new(repository);
        let result = user_register.execute(UserRegisterRequest::new(uuid, email, pwd, confirm_pwd)).await;
        if !result.get_success(){
            HttpResponse::BadRequest().json(result)
        } else {
            HttpResponse::Created().json(result)
        }

    }
}
