use actix_web::{body::MessageBody, http::header, App};
use testing_api::paths::*;

#[actix_web::test]
async fn test_plain_hello() {
    test_hello_func("/hello/plain/test", "Hello test Your A Monkey! 🦍", hello).await;
}

#[actix_web::test]
async fn test_qpram_hello() {
    test_hello_func("/hello/qparams?name=hello", "hello", qparams_hello).await;
}

#[actix_web::test]
async fn test_qpram_hello_invalid() {
    test_hello_func(
        "/hello/qparams?e=a",
        "Query deserialize error: missing field `name`",
        qparams_hello,
    )
    .await;
    test_hello_func(
        "/hello/qparams",
        "Query deserialize error: missing field `name`",
        qparams_hello,
    )
    .await;
}

async fn test_hello_func<F: actix_web::dev::HttpServiceFactory + 'static>(
    url: &str,
    body: &str,
    service: F,
) {
    let app = actix_web::test::init_service(App::new().service(service)).await;

    let request = actix_web::test::TestRequest::default()
        .uri(url)
        .insert_header(header::ContentType::plaintext())
        .to_request();

    let response = actix_web::test::call_service(&app, request).await;

    let body_string = String::from_utf8(
        actix_web::test::read_body(response)
            .await
            .try_into_bytes()
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    assert!(body == body_string.as_str())
}

#[actix_web::test]
async fn test_json_hello() {
    let name = "bob";
    let app = actix_web::test::init_service(App::new().service(json_hello)).await;

    let request = actix_web::test::TestRequest::default()
        .uri(format!("/hello/json/{name}").as_str())
        .insert_header(header::ContentType::json())
        .to_request();

    let response = actix_web::test::call_service(&app, request).await;

    let json: Message = actix_web::test::read_body_json(response).await;

    assert!(
        json == Message {
            hello: format!("Hello {name} Your A Super Monkey! 🙊")
        }
    )
}
