mod v1;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(v1::config);
}