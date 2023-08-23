use actix_web::{web, App, HttpServer, Responder, HttpResponse};

mod routes {
    pub mod v1 {
        pub mod user;
    }
}

mod utils {
    pub mod get_image;
}

mod services {
    pub mod user_service;
}

mod models {
    pub mod user;
}

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
            .service(web::scope("/v1/user").configure(routes::v1::user::config))
            .default_service(
                web::route().to(get_not_found)
            )
    })
    .bind(format!("0.0.0.0:{}", port))
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