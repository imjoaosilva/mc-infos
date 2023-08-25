mod user;
mod preview;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .configure(user::config)
            .configure(preview::config)
    );
}