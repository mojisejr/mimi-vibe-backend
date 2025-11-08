use actix_web::test;

use mimi_backend::create_app;

#[actix_rt::test]
async fn health_ok() {
    let app = test::init_service(create_app()).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
