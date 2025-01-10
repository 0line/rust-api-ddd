use crate::scope::healthy::healthcheck::StatusController;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::{from_value, Value};
use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::server::AppState;

#[get("/healthy")]
async fn run(req: HttpRequest, body: Option<web::Json<Value>>,  data: web::Data<AppState>) -> HttpResponse {
    let body = body.map(|json| json.into_inner()).unwrap_or(Value::Null);
    match from_value(body) {
        Ok(body) => {
            <StatusController as Controller>::run(req, body, data).await
        },
        Err(_) => {
            let result = APIResponse::new(false, Some("Error parsing body".to_string()), None, None);
            HttpResponse::BadRequest().json(result)
        }
    }
}