use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mc_infos::routes::v1;
use serde_json::json;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT")
        .unwrap_or(String::from("3000"))
        .parse::<u16>()
        .expect("Invalid port number");

    let server =     HttpServer::new(|| {
        App::new()
            .service(web::scope("/v1/user").configure(v1::user::config))
            .default_service(
                web::route().to(get_not_found)
            )
    })
    .bind(format!("127.0.0.1:{}", port))
    .expect("🔒 Connection error...")
    .run();

    println!("🚀 Server running at http://127.0.0.1:{}", port);

    server.await.unwrap();
}

async fn get_not_found() -> impl Responder {
    HttpResponse::NotFound().body(json!({
        "status": 404,
        "message": "Not Found"
    }).to_string())
}