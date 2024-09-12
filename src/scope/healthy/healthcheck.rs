use future::Future;
use std::future;
use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{HttpRequest, HttpResponse};
use serde_json::Value;


pub struct StatusController {}

impl Controller for StatusController {
    async fn run(_req:HttpRequest, _body: Value) -> HttpResponse {
        let result = APIResponse::new(true, Some("todo ok".to_string()), None, None);
        HttpResponse::Ok().json(result)
    }
}
