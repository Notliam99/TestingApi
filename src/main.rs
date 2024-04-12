use actix_web::{middleware::Logger, App, HttpServer};
use testing_api::{args, paths};

#[actix_web::main]
async fn main() {
    // init the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Port Helper
    let port = args::check_port();

    // Start the api server
    HttpServer::new(|| {
        App::new()
            .service(paths::hello)
            .service(paths::json_hello)
            .service(paths::qparams_hello)
            .wrap(Logger::new("Ip: ( %a ), Path: ( %U ), Latency: ( %Dms )")) // Logging
    })
    .bind(("127.0.0.1", port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
