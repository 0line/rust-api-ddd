use crate::shared::infraestruture::router::healthy;
use actix_web::web;
pub fn register(cfg: &mut web::ServiceConfig) {
    //cfg.route("/healthy", web::get().to(healthy::run));
    cfg.service(healthy::run);
}
