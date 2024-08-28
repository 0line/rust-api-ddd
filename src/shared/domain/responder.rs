use std::option::Option;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct ErrorOp {
    string: String,
    vector: Vec<serde_json::Value>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct APIResponse {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<Vec<serde_json::Value>>,
    pub error: Option<Vec<serde_json::Value>>,
}

impl APIResponse {
    pub fn new(
        success: bool,
        message: Option<String>,
        data: Option<Vec<serde_json::Value>>,
        error: Option<Vec<serde_json::Value>>,
    ) -> Self {
       Self {
            success,
            message,
            data,
            error,
       }
    }
}
