use actix_web::web;
use crate::handlers::shipping_handler::get_shipping_handler;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(get_shipping_handler);
}