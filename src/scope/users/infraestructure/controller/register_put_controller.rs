use std::rc::Rc;
use crate::shared::domain::responder::APIResponse;
use crate::shared::infraestruture::controllers::controller::Controller;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse, Result};
use serde_json::{json, to_string, Value};
use crate::scope::users::application::user_register::UserRegisterService;

/*pub struct RegisterController<R> {
    user_service: web::Data<UserRegisterService<R>>,
}

impl<R> RegisterController<R> {
    pub fn new(user_service: web::Data<UserRegisterService<R>>) -> Self {
        RegisterController { user_service }
    }
}*/
// impl Controller for RegisterController {
//     async fn run(_req:HttpRequest, body: Value) -> HttpResponse {
// 
//         let mut uuid = body.get("uuid").is_some().to_string();
//         let email = body.get("email").is_some().to_string();
//         let pwd = body.get("pwd").is_some().to_string();;
//         /*let mut user_repository_mock = MockUserRepository::new();
//         let user = UserRegister::new(Rc::new(user_repository_mock));
//         let user_request = User { uuid,   email: UserEmail::new(&email).unwrap(), pwd };
//         match user.execute(user_request).await {
//             Ok(user) => Ok(HttpResponse::Created().json(user)),
//             Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e))),
//         }
//        */
//         let result = APIResponse::new(false, Some("todo ok".to_string()), None, None);
//         HttpResponse::Ok().json(result)
//     }
// }
