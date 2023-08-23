use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::services::user_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{username}").route(web::get().to(get_name)));
}

async fn get_name(info: web::Path<String,>) -> impl Responder {

    let user = user_service::get_user_info(info.to_string()).await;

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
}
