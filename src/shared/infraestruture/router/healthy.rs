use crate::scope::healthy::healthcheck::StatusController;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::Value;
#[get("/healthy")]
async fn run(req: HttpRequest, body: web::Json<Value>) -> HttpResponse {
    return <StatusController as Controller>::run(
        req,
        body.into_inner()
    ).await;
}
