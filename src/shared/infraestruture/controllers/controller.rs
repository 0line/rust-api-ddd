use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, Result};
pub trait Controller {
    fn run(_req: HttpRequest) -> Result<HttpResponse, Error>;
}
