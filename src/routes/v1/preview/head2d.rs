use actix_web::{web, HttpResponse, Responder};
use crate::services::{user_service, preview_service};
use serde_json::json;

pub async fn get_2dhead(info: web::Path<String>) -> impl Responder {
    if let Some(user) = user_service::get_user_info(info.to_string()).await {
        let image = user_service::get_image(user.textures.skin).await;

        if let Some(bytes) = image {
            let bytes = preview_service::head2d_manipulation(bytes);

            return HttpResponse::Ok()
                .content_type("image/png")
                .body(bytes);
        }

        HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
    else {
        HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
}