use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::{Uuid};
use crate::scope::users::domain::user_email::UserEmail;
use crate::scope::users::domain::user_id::UserId;
use crate::scope::users::domain::user_pwd::UserPwd;
use crate::scope::users::domain::users_errors::UserError;
use crate::shared::domain::responder::APIResponse;

//Entity User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User{
    uuid: UserId,
    email: UserEmail,
    pwd: UserPwd,
}

impl User {
    fn new(uuid: UserId, email: UserEmail, pwd: UserPwd) -> Self {
       /* let uuid = UserId::new(uuid)?;
        let email = UserEmail::new(email)?;
        let pwd = UserPwd::new(pwd)?;*/
        Self {
            uuid,
            email,
            pwd
        }
    }

    pub fn create(uuid: String, email: String, pwd: String, confirmpwd: String) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();
        /*let uuid = UserId::new(uuid);
        let email = UserEmail::new(email);
        let pwd = UserPwd::new(pwd);*/

        let uuid = match UserId::new(uuid) {
            Ok(uuid) => Some(uuid),
            Err(err) => {
                errors.push(err.to_string());
                None
            }
        };

        let email = match UserEmail::new(email) {
            Ok(email) => Some(email),
            Err(err) => {
                errors.push(err.to_string());
                None
            }
        };

        let password = match UserPwd::new(pwd, confirmpwd) {
            Ok(password) => Some(password),
            Err(err) => {
                errors.push(err.to_string());
                None
            }
        };
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(Self::new(uuid.unwrap(), email.unwrap(), password.unwrap()))
        }
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
            "uuid": &self.uuid.get_value(),
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
    use crate::scope::users::domain::user;
    use super::*;

    //const UUID : UserId  = UserId::try_from("b92f6347-4d73-4427-8ed7-512f9d58738f").unwrap();
    const UUID:&str = "b92f6347-4d73-4427-8ed7-512f9d58738f";
    const EMAIL :&str   = "test@mail.com";
    const PWD:&str  = "Asdf123#";
    const CONFIRMPWD:&str  = "Asdf123#";

    #[test]
    fn should_create_the_expected_user() {
        let user = User::create(
            UUID.to_string(),
            EMAIL.to_string(),
            PWD.to_string(),
            CONFIRMPWD.to_string()
        );
        let result_user = user.unwrap();
        //println!("{:?}", result_user.get_user_to_json());
        assert_eq!(result_user.get_email(), EMAIL);
        assert_eq!(result_user.get_pwd(), PWD);
        assert_eq!(result_user.get_id().to_string(), UUID);
    }

    #[test]
    fn should_fail_id() {
        let uuid = "9878".to_string();
        match User::create(
           uuid,
           EMAIL.to_string(),
           PWD.to_string(),
           CONFIRMPWD.to_string()
       )
        {
            Ok(..) => assert!(false),
            Err(e) => {
                assert_eq!(&e[0], "The value id is invalid")
            }
       }
    }

    #[test]
    fn should_fail_email() {
        let email = "test".to_string();
        match User::create(
            UUID.to_string(),
            email,
            PWD.to_string(),
            CONFIRMPWD.to_string()
        )
        {
            Ok(..) => assert!(false),
            Err(e) => {
                //println!("{:?}", &e[0]);
                assert_eq!(&e[0], "The input  <email> must have the format example@mail.com")
            }
        }
    }

    #[test]
    fn should_fail_email_and_pwd() {
        let email = "test".to_string();
        let pwd = "123".to_string();
        let confirmpwd = "325".to_string();
        match User::create(
            UUID.to_string(),
            email,
            pwd,
            confirmpwd
        )
        {
            Ok(..) => assert!(false),
            Err(e) => {
                //println!("{:?}", &e);
                assert_eq!(&e[0], "The input  <email> must have the format example@mail.com");
                assert_eq!(&e[1], "The pwd value of is too short, it must contain at least 8 characters.");
            }
        }
    }

}