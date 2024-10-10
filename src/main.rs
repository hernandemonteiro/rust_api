use actix_web::{get, post, web, App, HttpServer, Responder};
use serde_json::json;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn index() -> impl Responder {
    web::Json(json!({"message": "Hello from my first api!"}))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    format!("Request body: {}", req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(index).service(echo))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
