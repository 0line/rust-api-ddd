use crate::scope::healthy::healthcheck::StatusController;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Result};
use serde_json::Value;
#[get("/healthy")]
async fn run(req: HttpRequest, body: web::Json<Value>) -> Result<HttpResponse,Error> {
    return <StatusController as Controller>::run(
        req,
        body.into_inner()
    );
}
