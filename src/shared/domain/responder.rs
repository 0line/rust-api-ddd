use std::option::Option;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct APIResponse {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<Vec<serde_json::Value>>,
    pub error: Option<Vec<String>>,
}

impl APIResponse {
    pub fn new(
        success: bool,
        message: Option<String>,
        data: Option<Vec<serde_json::Value>>,
        error: Option<Vec<String>>,
    ) -> Self {
       Self {
            success,
            message,
            data,
            error
       }
    }

    pub fn get_success(&self) -> bool {
        self.success
    }
}
