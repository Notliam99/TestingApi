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

async fn check_port() -> u16 {
    let default_port: u16 = 8080;
    let args: Vec<String> = std::env::args().collect();
    // iterate through arguements checking for port arguement
    for (_index, arg) in args.iter().enumerate() {
        if arg == "-p" || arg == "--port" {
            // check if a arguement exists after -p
            if let Some(port_str) = args.get(_index + 1) {
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
    let port = check_port().await;
    HttpServer::new(|| App::new().service(hello).service(json_hello))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
