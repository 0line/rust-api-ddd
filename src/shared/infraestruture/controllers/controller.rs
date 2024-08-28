use actix_web::{Error, HttpRequest, HttpResponse, Result};
use serde_json::Value;

pub trait Controller {
    fn run(req: HttpRequest, body:Value) -> Result<HttpResponse, Error>;
}
