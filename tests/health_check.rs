//! tests/health_check.rs

use rust_rest_api::health_check;

use actix_web::{http::header::ContentType, test, web, App};

#[actix_web::test]
async fn health_check_works() {
    let app = test::init_service(App::new().route("/", web::get().to(health_check))).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
