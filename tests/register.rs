use actix_web::{test, App};
use serde_json::Value;

#[macro_use]
extern crate serde_json;
use apiusers::shared::{domain::responder::APIResponse, infraestruture::router::register_users};

#[actix_rt::test]
async fn test_register_user() {
    let mut app = test::init_service(App::new().service(register_users::run)).await;
    let data = json!({
        "uuid":"b92f6347-4d73-4427-8ed7-512f9d58738f",
        "email":"test@mail.com",
        "pwd": "1234567890"
    });
    let req = test::TestRequest::put().uri("/register").set_json(&data).to_request();
    let resp = test::call_service(&mut app, req).await;
    //let result: APIResponse = test::read_body_json(resp).await;
    let result: Value = test::read_body_json(resp).await;
    print!("{:?}", result);
    //let expected: Option<String> = Some(result.message.unwrap().to_spetring());
    //let actual: Option<String> = Some("todo ok".to_string());
    //assert_eq!(expected, actual);
}
