use crate::shared::infraestruture::router::{healthy, register_users};
use actix_web::web;
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(healthy::run);
    cfg.service(register_users::run);
}
