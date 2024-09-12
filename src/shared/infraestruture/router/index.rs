use crate::shared::infraestruture::router::{healthy, users};
use actix_web::web;
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(healthy::run);
    cfg.service(users::run);
}
