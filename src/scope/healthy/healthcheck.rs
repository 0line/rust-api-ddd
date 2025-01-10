use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;
use crate::shared::infraestruture::server::AppState;

pub struct StatusController {}

impl Controller for StatusController {
    async fn run(_req:HttpRequest, _body: Value, _data: web::Data<AppState>) -> HttpResponse {
        let result = APIResponse::new(true, Some("todo ok".to_string()), None, None);
        HttpResponse::Ok().json(result)
    }
}
