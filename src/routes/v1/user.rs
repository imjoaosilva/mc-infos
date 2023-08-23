use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::services::user_service;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/profile/{username}").route(web::get().to(get_name))
    )
    .service(
        web::resource("/cape/{username}").route(web::get().to(get_cape))
    );
}

async fn get_name(info: web::Path<String,>) -> impl Responder {

    let user =  user_service::get_user_info(info.to_string()).await;

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
}


async fn get_cape(info: web::Path<String,>) -> impl Responder {

    let user =  user_service::get_user_info(info.to_string()).await;

    match user {
        Some(user) =>{
            let base64 = user_service::get_image_from_url(user.textures.cape).await;
            if let Some(bytes) = base64 {

                return HttpResponse::Ok()
                    .content_type("image/png")
                    .body(bytes);
            }

            HttpResponse::NotFound().body(json!({
                "status": 404,
                "message": "Not Found"
            }).to_string())
        },
        None => HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
}