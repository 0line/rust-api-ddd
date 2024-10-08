use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::{Uuid};
use crate::scope::users::domain::user_email::UserEmail;
use crate::scope::users::domain::user_id::UserId;
use crate::scope::users::domain::user_pwd::UserPwd;
use crate::scope::users::domain::users_errors::UserError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    uuid: UserId,
    email: UserEmail,
    pwd: UserPwd,
}

impl User {
    pub fn new(uuid: UserId, email: UserEmail, pwd: UserPwd) -> Result<Self, UserError> {
        Ok(Self {
            uuid,
            email,
            pwd
        })
    }
    pub fn get_id(&self) -> &Uuid {
        &self.uuid.get_value()
    }

    pub fn get_email(&self) -> &str {
        &self.email.get_value()
    }

    pub fn get_pwd(&self) -> &str {
        &self.pwd.get_value()
    }

    pub fn get_user_to_json(&self) -> Value {
        json!({
            "uuid": self.uuid.get_value(),
            "email": self.email.get_value(),
            "pwd": self.pwd.get_value()
        })
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}",
               self.uuid.get_value(),
               self.email.get_value()
        )
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::scope::users::domain::user;
    use super::*;

    //const UUID : UserId  = UserId::try_from("b92f6347-4d73-4427-8ed7-512f9d58738f").unwrap();
    const UUID:&str = "b92f6347-4d73-4427-8ed7-512f9d58738f";
    const EMAIL :&str   = "test@mail.com";
    const PWD:&str  = "Asdf123#";

    #[test]
    fn should_create_the_expected_user() {
        let user = User::new(
            UserId::new(UUID.to_string()).unwrap(),
            UserEmail::new(EMAIL.to_string()).unwrap(),
            UserPwd::new(PWD.to_string()).unwrap()
        ).unwrap();
        println!("{:?}", &user);
        assert_eq!(user.get_email(), EMAIL);
        assert_eq!(user.get_id().to_string(), UUID);
        assert_eq!(user.get_pwd(), PWD);
    }

    #[test]
    fn should_fail_id() {
        let uuid = "9878".to_string();
        match UserId::new(uuid) {
            Ok(uuid) => {
                let user = User::new(
                    uuid,
                    UserEmail::new(EMAIL.to_string()).unwrap(),
                    UserPwd::new(PWD.to_string()).unwrap()
                );
                println!("{:?}", &user);
            },
            Err(e) => {
                println!("Error: {}", e);
                assert_eq!(e.to_string(), "The value <id> is invalid");
            }
        }

        //assert_eq!(result.unwrap_err(), "The value <9878> is invalid");
    }

    // #[test]
    // fn should_fail_email() {
    //
    //     let result = User::new(
    //         UserId::new("9878".to_string()).unwrap(),
    //         UserEmail::new(EMAIL.to_string()).expect("The value <mail.mail.com> is invalid"),
    //         UserPwd::new(PWD.to_string()).expect("The value <mail.mail.com> is invalid")
    //     );
    //     // match result {
    //     //     Ok(r) => println!("{:?}", r),
    //     //     Err(e) => println!("{:?}", e)
    //     // }
    //    // println!("{:?}", result.unwrap().get_id());
    //    // assert_eq!("The value <mail.mail.com> is invalid", result.unwrap_err());
    //     if let Err(e) = result {
    //         println!("{:?}", e.to_string());
    //         //assert!(e.contains("The value <mail.mail.com> is invalid"));
    //     }
    // }

    #[test]
    fn should_fail_pwd() {
        if let Err(err) = User::new(
            UserId::new(UUID.to_string()).unwrap(),
            UserEmail::new(EMAIL.to_string()).unwrap(),
            UserPwd::new("123456".to_string()).unwrap()
        ){
            match err {
                UserError::TooShort(field, _) => {
                    assert_eq!(field, "pwd");
                },
                _ => {
                    assert!(false);
                }
            }
        }
        //println!("{:?}", result);
        //assert_eq!("The value <mail.mail.com> is invalid", result.unwrap_err());
    }
}