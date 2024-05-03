use actix_web::{get, web, HttpResponse, Responder, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

// returns path parameter "name" as plain text
#[utoipa::path(
    responses(
        (status = 200, description = "Valid Name Entered"),
    ),
)]
#[get("/hello/plain/{name}")]
pub async fn hello(name: web::Path<String>) -> impl Responder {
    // responds with the below text and path parameter "name" which the user chooses
    format!("Hello {name} Your A Monkey! 🦍")
}

// Creates message object for json responses
#[derive(Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct Message {
    pub hello: String,
}

// returns path parameter "name" as json object
#[utoipa::path(
    responses(
        (status = 200, description = "Valid Name Entered", body = Message)
    ),
)]
#[get("/hello/json/{name}")]
pub async fn json_hello(name: web::Path<String>) -> Result<web::Json<Message>> {
    // creates object message
    let message = Message {
        hello: format!("Hello {} Your A Super Monkey! 🙊", name.to_string()),
    };
    // responds with the json object 'Message'
    Ok(web::Json(message))
}

#[derive(Deserialize, IntoParams)]
pub struct HelloParams {
    pub name: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Valid Name Pram"),
        (status = 400, description = "Invalid Request Missing `name` Param")
    ),
    params(
        HelloParams
    )
)]
#[get("/hello/qparams")]
pub async fn qparams_hello(params: web::Query<HelloParams>) -> HttpResponse {
    // responds with the json object 'Message
    HttpResponse::Ok().body(format!("{}", params.name))
}

pub async fn catch_all() -> HttpResponse {
    HttpResponse::NotFound().body(format!("404!"))
}
