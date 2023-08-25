mod cape;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/preview")
    .route("/cape/{username}", web::get().to(cape::get_cape))
  );
}
