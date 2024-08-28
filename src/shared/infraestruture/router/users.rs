use actix_web::{post, web, Error, HttpRequest, HttpResponse};
use actix_web::web::Json;
use serde_json::Value;
use crate::scope::users::infraestructure::register::RegisterController;
use crate::shared::infraestruture::controllers::controller::Controller;


#[post("/register")]
async fn run(req: HttpRequest, body: web::Json<Value>) -> actix_web::Result<HttpResponse, Error> {
    return <RegisterController as Controller>::run(
        req,
        body.into_inner(),
    );
}