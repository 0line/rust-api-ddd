#[macro_use]
extern crate serde_json;
//use apiusers::shared::{domain::responder::APIResponse, infraestruture::router::register_users};
//#[actix_rt::test]
use serde_json::json;
use crate::scope_test::users::domain::user_mother::UserMother;

pub async fn test_register_user() {
    //let mut app = test::init_service(App::new().service(register_users::run)).await;
    let user = UserMother::new();
    let data = json!(user);
    println!("{:?}", data);
}