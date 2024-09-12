use std::{fs};
use std::fs::{File};
use std::io::{Write};
use std::path::{Path};
use async_trait::async_trait;
use std::sync::RwLock;
use serde_json::{to_string};
use crate::scope::users::domain::user::User;
use crate::scope::users::domain::user_repository::UserRepository;
use crate::shared::domain::responder::APIResponse;

// users: RwLock<Vec<User>>,
pub struct MemoryRepository {
    users: RwLock<Vec<User>>,
}

impl MemoryRepository {
    pub fn new() -> Self {
        MemoryRepository {
            users: RwLock::new(Vec::new()),
        }
    }

    pub fn save_json_data<T: serde::Serialize>(file_path: &str, data: &T, errors: &mut Vec<String>) {
        // Asegurarse de que el directorio existe
        let dir_path = Path::new(file_path).parent().unwrap();
        if !dir_path.exists() {
            if let Err(e) = fs::create_dir_all(dir_path) {
                errors.push(format!("Error al crear el directorio: {}", e));
                return;
            }
        }

        // Convertir datos a JSON
        let json_data = match to_string(data) {
            Ok(json) => json,
            Err(e) => {
                errors.push(format!("Error al convertir los datos a JSON: {}", e));
                return;
            }
        };

        // Guardar el JSON en el archivo
        match File::create(file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_data.as_bytes()) {
                    errors.push(format!("Error al escribir en el archivo JSON: {}", e));
                }
            }
            Err(e) => errors.push(format!("Error al crear el archivo JSON: {}", e)),
        }
    }
}
#[async_trait]
impl UserRepository for MemoryRepository {
    async fn save(&self, user: User) -> APIResponse {
        let mut error_vec: Vec<String> = Vec::new();
        let file_path = "./data/users.json";
        let data = &user.get_user_to_json();
        MemoryRepository::save_json_data(file_path, &data, &mut error_vec);
        if !error_vec.is_empty() {
            APIResponse::new(
                false,
                Some("Ocurrio un error".to_string()),
                None,
                Some(error_vec));
        }
        let concatenated = format!("Se creó correctamente el usuario {}", user.get_email());
        APIResponse::new(
            true,
            Some(concatenated.to_string()),
            None,
            None)
    }
}