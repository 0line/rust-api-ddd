use actix_web::{put, web, Error, HttpRequest, HttpResponse};
use serde_json::Value;
use crate::scope::users::domain::user_repository::UserRepository;
//use crate::scope::users::infraestructure::controller::register_put_controller::RegisterController;
use crate::shared::infraestruture::controllers::controller::Controller;


/*#[put("/register")]
async fn run(req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    return <RegisterController as Controller>::run(
        req,
        body.into_inner(),
    ).await;
}*/

