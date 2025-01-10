use actix_web::{put, web,  HttpRequest, HttpResponse};
use serde_json::Value;
use crate::scope::users::infraestructure::controller::register_put_controller::RegisterController;
use crate::scope::users::infraestructure::persistence::PostgresqlUserRepository::PostgresqlUserRepository;
use crate::shared::infraestruture::controllers::controller::Controller;
use crate::shared::infraestruture::server::AppState;
//use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;

#[put("/register")]
async fn run(req: HttpRequest, body: web::Json<Value>, data: web::Data<AppState>) -> HttpResponse {
    return<RegisterController<PostgresqlUserRepository> as Controller>::run(
        req,
        body.into_inner(),
        data
    ).await;
}

