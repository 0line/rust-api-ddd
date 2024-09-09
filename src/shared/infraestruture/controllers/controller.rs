use actix_web::{HttpRequest, HttpResponse};
use serde_json::Value;

pub trait Controller {
    async fn run(req: HttpRequest, body:Value) -> HttpResponse;
}
