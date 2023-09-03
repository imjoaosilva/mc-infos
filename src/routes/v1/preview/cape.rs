use actix_web::{web, HttpResponse, Responder};
use crate::services::{user_service, preview_service};
use serde_json::json;

pub async fn get_cape(info: web::Path<String>) -> impl Responder {
    if let Some(user) = user_service::get_user_info(info.to_string()).await {
        
        let cape = user.textures.cape;

        if cape.is_none() {
            return HttpResponse::NotFound().body(json!({
                "status": 404,
                "message": "Not Found"
            }).to_string())
        }

        let image = user_service::get_image(cape.unwrap().url).await;

        if let Some(bytes) = image {
            let bytes = preview_service::cape_manipulation(bytes);
            
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