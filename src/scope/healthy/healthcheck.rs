use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{Error,HttpRequest, HttpResponse, Result};
use serde_json::Value;

pub struct StatusController {}

impl Controller for StatusController {
    fn run(_req:HttpRequest, body: Value) -> Result<HttpResponse, Error> {
        let result = APIResponse::new(true, Some("todo ok".to_string()), None, None);
        Ok(HttpResponse::Ok().json(result))
    }
}
