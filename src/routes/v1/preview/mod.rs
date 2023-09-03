mod cape;
mod head2d;
mod head3d;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::scope("/preview")
    .route("/cape/{username}", web::get().to(cape::get_cape))
    .route("/2dhead/{username}", web::get().to(head2d::get_2dhead))
    .route("/head3d/{username}", web::get().to(head3d::get_3dhead))
  );
}
