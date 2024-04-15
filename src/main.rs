use actix_web::{middleware::Logger, App, HttpServer};
use testing_api::{args, paths};

#[actix_web::main]
async fn main() {
    // init the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Start the api server with port helper (args::check_port())
    HttpServer::new(|| {
        App::new()
            .service(paths::hello)
            .service(paths::json_hello)
            .service(paths::qparams_hello)
            .wrap(Logger::new("Ip: ( %a ), Path: ( %U ), Latency: ( %Dms )")) // Logging
    })
    .bind(("127.0.0.1", args::check_port().unwrap()))
    .unwrap()
    .run()
    .await
    .unwrap()
}
