use std::future::IntoFuture;
use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{web, Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub struct RegisterController {}


#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    uuid: String,
    email: String

}

impl Controller for RegisterController {
    fn run(_req:HttpRequest, body: Value) -> Result<HttpResponse, Error> {

        //let mut body = _req.pa
        let result = APIResponse::new(true, Some("todo ok".to_string()), None, None);
        Ok(HttpResponse::Ok().json(body))
    }
}
