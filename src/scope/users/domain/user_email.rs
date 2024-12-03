use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::scope::users::domain::users_errors::UserError;

//Value Object UserEmail
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserEmail {
    value: String,
}


impl UserEmail {
    pub fn new(value: String) -> Result<Self, UserError> {
        if value.is_empty() {
            return Err(UserError::Empty("email".to_string()));
        }
        if !Self::ensure_email_format(&value){
            return Err(UserError::InvalidFormat("email".to_string(),"example@mail.com".to_string()))
        }
        Ok(
            Self{
                value
            }
        )
    }

    fn ensure_email_format(value: &str) -> bool {
        let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
        email_regex.is_match(&value)
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}


