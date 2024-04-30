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
            .default_service(actix_web::web::to(paths::catch_all))
            .wrap(
                Logger::new("Response: [%s], Ip: ( %{r}a ), Path: ( %U ), Latency: ( %Dms )")
                    .log_target("Http_Logs"),
            ) // Logging
    })
    .bind(("0.0.0.0", args::arg_port()))
    .unwrap()
    .run()
    .await
    .unwrap()
}
