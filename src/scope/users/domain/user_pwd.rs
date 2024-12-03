use serde::{Deserialize, Serialize};
use UserError::PasswordsDoNotMatch;
use crate::scope::users::domain::users_errors::UserError;

//Value Object UserPwd
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPwd {
    value: String,
}

impl UserPwd {
    // Constructor que valida la contraseña al crear una instancia
    pub fn new(password: String, confirmpwd: String) -> Result<Self, UserError> {
        if password.is_empty() {
            return Err(UserError::Empty("pwd".to_string()));
        }

        if password.chars().count() < 8 {
            return Err(UserError::TooShort("pwd".to_string(), 8));
        }

        if !Self::is_valid_password(&password) {
            return Err(UserError::InvalidFormat("pwd".to_string(), "Uppercase, lowercase, digit and special character (!@#$%^&*()_+[]{}|;:',.<>?/~`)".to_string()));
        }

        if  !Self::ensuere_passwords_match(&password, &confirmpwd) {
            return Err(PasswordsDoNotMatch);
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


    pub fn ensuere_passwords_match(password: &str, confirm_password: &str) -> bool {
        password == confirm_password
    }

    // Método para obtener la contraseña (por motivos de ejemplo)
    pub fn get_value(&self) -> &str {
       &self.value
    }

    pub fn encrypt_password(&self) -> String {
        // Encriptar la contraseña
        self.value.clone()
    }

    pub fn decrypt_password(&self) -> String {
        // Desencriptar la contraseña
        self.value.clone()
    }
}