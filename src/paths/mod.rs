use actix_web::{get, web, Responder, Result};
use serde::Serialize;

// returns path parameter "name" as plain text
#[get("/hello/plain/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    // responds with the below text and path parameter "name" which the user chooses
    format!("Hello {name} Your A Monkey! 🦍")
}

// Creates message object for json responses
#[derive(Serialize)]
pub struct Message {
    hello: String,
}

// returns path parameter "name" as json object
#[get("/hello/json/{name}")]
pub async fn json_hello(name: web::Path<String>) -> Result<web::Json<Message>> {
    // creates object message
    let message = Message {
        hello: format!("Hello {} Your A Super Monkey! 🙊", name.to_string()),
    };
    // responds with the json object 'Message'
    Ok(web::Json(message))
}
