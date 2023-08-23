mod cape;
mod profile;
mod skin;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/user")
    .route("/cape/{username}", web::get().to(cape::get_cape))
    .route("/profile/{username}", web::get().to(profile::get_profile))
    .route("/skin/{username}", web::get().to(skin::get_skin))
  );
}
