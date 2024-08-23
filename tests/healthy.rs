use actix_web::{test, App};
use apiusers::shared::{domain::responder::APIResponseSuccess, infraestruture::router::healthy};

#[actix_rt::test]
async fn test_run() {
    let app = test::init_service(App::new().service(healthy::run)).await;
    let req = test::TestRequest::get().uri("/healthy").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let resutl: APIResponseSuccess = test::read_body_json(resp).await;
    //print!("{:?}", resutl);
    assert_eq!(resutl.message, "todo ok");
}
