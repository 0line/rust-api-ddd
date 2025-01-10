use actix_web::{web, HttpRequest, HttpResponse};
use serde_json::Value;
use crate::shared::infraestruture::server::AppState;

pub trait Controller {
    async fn run(req: HttpRequest, body:Value, data: web::Data<AppState>) -> HttpResponse;

}
