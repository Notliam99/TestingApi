use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use testing_api::{args, paths};
use utoipa::{self, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() {
    // init the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            paths::hello,
            paths::json_hello,
            paths::qparams_hello,
            paths::protected_hello
        ),
        components(schemas(paths::Message))
    )]
    struct ApiDoc;

    // Start the api server with port helper (args::check_port())
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/protected")
                    .service(paths::protected_hello)
                    .service(paths::protected_hello_index),
            )
            .service(paths::hello)
            .service(paths::json_hello)
            .service(paths::qparams_hello)
            .service(
                SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
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
