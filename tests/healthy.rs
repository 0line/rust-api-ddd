use actix_web::{test, App};
use apiusers::shared::{domain::responder::APIResponse, infraestruture::router::healthy};
use actix_web::http::header::{HeaderMap, HeaderValue};
#[actix_rt::test]
async fn test_healthy() {
    let app = test::init_service(App::new().service(healthy::run)).await;
    let mut hm: HeaderMap = HeaderMap::new();
    hm.insert(actix_web::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let req = test::TestRequest::get()
        .insert_header((
        actix_web::http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
        ))
        .uri("/healthy")
        .to_request();
    let resp = test::call_service(&app, req).await;
    let result: APIResponse = test::read_body_json(resp).await;
    print!("{:?}", result);
   let expected: Option<String> = Some(result.message.unwrap().to_string());
    println!("{:?}", expected);
   let actual: Option<String> = Some("todo ok".to_string());
   assert_eq!(expected, actual);
}
