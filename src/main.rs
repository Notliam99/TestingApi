use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[get("/hello/plain/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {name} Your A Monkey! 🦍")
}

#[derive(Serialize)]
struct Message {
    hello: String,
}

#[get("/hello/json/{name}")]
async fn json_hello(name: web::Path<String>) -> Result<web::Json<Message>> {
    let message = Message {
        hello: format!("Hello {} Your A Super Monkey! 🙊", name.to_string()),
    };

    Ok(web::Json(message))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(json_hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
