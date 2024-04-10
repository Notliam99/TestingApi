use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

// returns path parameter "name" as plain text
#[get("/hello/plain/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    // responds with the below text and path parameter "name" which the user chooses 
    format!("Hello {name} Your A Monkey! 🦍")
}

// Creates message object for json responses
#[derive(Serialize)]
struct Message {
    hello: String,
}

// returns path parameter "name" as json object
#[get("/hello/json/{name}")]
async fn json_hello(name: web::Path<String>) -> Result<web::Json<Message>> {
    // creates object message
    let message = Message {
        hello: format!("Hello {} Your A Super Monkey! 🙊", name.to_string()),
    };
    // responds with the json object 'Message' 
    Ok(web::Json(message))
}

// this function checks for the -p or --port flags and returns a unsigned 16 int aka the arg
fn check_port() -> u16 {
    let default_port: u16 = 8080;
    let args: Vec<String> = std::env::args().collect();
    // iterate through arguements checking for port arguement
    for (index, arg) in args.iter().enumerate() {
        if arg == "-p" || arg == "--port" {
            // check if a arguement exists after -p
            if let Some(port_str) = args.get(index + 1) {
                /* if string converts to unsigned 16 int then return the port
                otherwise error etc. default to default_port */
                if let Ok(port) = port_str.parse::<u16>() {
                    println!("Port set to {}", port);
                    return port;
                } else {
                    println!("No valid port specified defaulting to {}", default_port);
                    return default_port;
                }
            } 
        }
    }
    // if there are no arguments return default port
    return default_port;
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = check_port();
    // start the api server
    HttpServer::new(|| App::new().service(hello).service(json_hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
