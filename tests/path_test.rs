use std::str::FromStr;

use actix_web::{body::MessageBody, http::header, web::Json, App};
use testing_api::paths::*;

#[actix_web::test]
async fn test_plain_hello() {
    let name = "Jim";

    let app = actix_web::test::init_service(App::new().service(hello)).await;

    let request = actix_web::test::TestRequest::default()
        .uri(format!("/hello/plain/{name}").as_str())
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

    assert!(format!("Hello {name} Your A Monkey! 🦍") == body_string)
}

#[actix_web::test]
async fn test_json_hello() {
    let name = "bob";

    let app = actix_web::test::init_service(App::new().service(json_hello)).await;

    let request = actix_web::test::TestRequest::default()
        .uri(format!("/hello/plain/{name}").as_str())
        .insert_header(header::ContentType::json())
        .to_request();

    let response = actix_web::test::call_service(&app, request).await;

    let json: Message = actix_web::test::read_body_json(response).await;

    print!("{:?}", json);

    assert!(
        json == Message {
            hello: format!("Hello {name} Your A Super Monkey! 🙊")
        }
    )
}
