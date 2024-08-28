use actix_web::{test, App};
use apiusers::shared::{domain::responder::APIResponse, infraestruture::router::healthy};

#[actix_rt::test]
async fn test_healthy() {
    let app = test::init_service(App::new().service(healthy::run)).await;
    let req = test::TestRequest::get().uri("/healthy").to_request();
    let resp = test::call_service(&app, req).await;
    let result: APIResponse = test::read_body_json(resp).await;
    print!("{:?}", result);
    let expected: Option<String> = Some(result.message.unwrap().to_string());
    let actual: Option<String> = Some("todo ok".to_string());
    assert_eq!(expected, actual);
}
