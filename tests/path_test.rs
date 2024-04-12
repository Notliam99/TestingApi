use actix_web::{body::MessageBody, http::header, App};
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

    let vec_u8 = response.into_body().try_into_bytes().unwrap().to_vec();

    assert!(format!("Hello {name} Your A Monkey! 🦍") == String::from_utf8(vec_u8).unwrap())
}
