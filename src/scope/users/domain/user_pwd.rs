use std::convert::TryFrom;
use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::value;
use uuid::Uuid;
use crate::scope::users::domain::user_id::UserId;

#[derive(Debug)]
pub enum PasswordError {
    Empty,
    TooShort,
    Invalid,
}
// Implementar el display para los errores de contraseña
impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::Empty => write!(f, "La contraseña no puede estar vacía."),
            PasswordError::TooShort => write!(f, "La contraseña debe tener al menos 8 caracteres."),
            PasswordError::Invalid => write!(f, "La contraseña no es válida."),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPwd {
    value: String,
}

impl UserPwd {
    // Constructor que valida la contraseña al crear una instancia
    pub fn new(password: String) -> Result<Self, PasswordError> {
        if password.is_empty() {
            return Err(PasswordError::Empty);
        }

        if password.chars().count() < 8 {
            return Err(PasswordError::TooShort);
        }

        if !Self::is_valid_password(&password) {
            return Err(PasswordError::Invalid);
        }

        Ok(UserPwd {
            value: password
        })
    }

    // Método privado para validar la contraseña
    fn is_valid_password(password: &str) -> bool {
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_digit(10));
        let has_special = password.chars().any(|c| "!@#$%^&*()_+[]{}|;:',.<>?/~`".contains(c));

        has_upper && has_lower && has_digit && has_special
    }

    // Método para obtener la contraseña (por motivos de ejemplo)
    pub fn get_value(&self) -> &str {
       &self.value
    }
}


/*#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Userpwd (String);

impl Userpwd {
    pub fn value(&self) -> &String {
        &self.0
    }
    pub fn is_valid_pwd(&self)-> bool {
        let has_upper = self.value().chars().any(|c| c.is_uppercase());
        let has_lower = self.value().chars().any(|c| c.is_lowercase());
        let has_digit = self.value().chars().any(|c| c.is_digit(10));
        let has_special = self.value().chars().any(|c| "!@#$%^&*()_+[]{}|;:',.<>?/~`".contains(c));
        has_upper && has_lower && has_digit && has_special
    }
}*/

/*impl TryFrom<String> for Userpwd {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.to_string().is_empty() {
            return Err("El password es requerido")
        }
        if value.chars().count() < 8 {
            return Err("El password debe contener al menos 8 caracteres");
        }

        Ok(Self(value.to_string()))
    }

}
*/
/*impl From<&str> for Userpwd {
    fn from(s: &str) -> Self {
        Userpwd(s.to_string())
    }
}*/