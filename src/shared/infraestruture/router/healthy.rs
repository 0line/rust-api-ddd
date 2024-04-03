use crate::scope::healthy::healthcheck::StatusController;
use crate::shared::infraestruture::controllers::controller::Controller;
//use agregator::{scope::healthy::healthcheck::StatusController};
//use crate::shared::domain::responder::APIResponseSuccess;
use actix_web::{get, web, Error, HttpRequest, HttpResponse, Responder, Result};
use serde_json::json;

#[get("/healthy")]
async fn run(req: HttpRequest) -> Result<HttpResponse, Error> {
    return <StatusController as Controller>::run(req);
    //status::run().await
    /* let mut result = APIResponseSuccess::new(true, "todo ok".to_string(), vec![]);

    Ok(HttpResponse::Ok().json(result)) */
}
