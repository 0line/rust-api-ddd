
mod scope_test;
use scope_test::users::application::user_register;

#[actix_rt::test]
async fn test_register_user() {
    user_register::test_register_user().await;
}