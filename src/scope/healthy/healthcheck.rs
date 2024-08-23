use crate::shared::domain::responder::APIResponseSuccess;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{Error, HttpRequest, HttpResponse, Result};
pub struct StatusController {
    run: fn(HttpRequest) -> Result<HttpResponse, Error>,
}

impl Controller for StatusController {
    fn run(_req: HttpRequest) -> Result<HttpResponse, Error> {
        let result = APIResponseSuccess::new(true, "todo ok".to_string(), vec![]);
        Ok(HttpResponse::Ok().json(result))
    }
}
