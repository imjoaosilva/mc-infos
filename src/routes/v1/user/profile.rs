use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service;
use serde_json::json;

pub async fn get_profile(info: web::Path<String>) -> impl Responder {
    let user =  user_service::get_user_info(info.to_string()).await;

    if let Some(user) = user {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body(json!({
            "status": 404,
            "message": "Not Found"
        }).to_string())
    }
}