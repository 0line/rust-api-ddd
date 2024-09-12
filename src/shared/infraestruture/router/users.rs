use actix_web::{put, web,  HttpRequest, HttpResponse};
use serde_json::Value;
use crate::scope::users::infraestructure::controller::register_put_controller::RegisterController;
use crate::shared::infraestruture::controllers::controller::Controller;
use crate::scope::users::infraestructure::persistence::memory::MemoryRepository;

#[put("/register")]
async fn run(req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    return<RegisterController<MemoryRepository> as Controller>::run(
        req,
        body.into_inner(),
    ).await;
}

