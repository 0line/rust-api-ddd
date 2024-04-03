use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(serde::Serialize)]
pub struct APIResponseSuccess {
    success: bool,
    message: String,
    data: Vec<serde_json::Value>,
}

#[derive(serde::Serialize)]
pub struct APIResponseError {
    success: bool,
    message: String,
    error: Vec<serde_json::Value>,
}

impl APIResponseSuccess {
    pub fn new(success: bool, message: String, data: Vec<serde_json::Value>) -> Self {
        Self {
            success,
            message,
            data,
        }
    }
}

impl APIResponseError {
    fn new(success: bool, message: String, error: Vec<serde_json::Value>) -> Self {
        Self {
            success,
            message,
            error,
        }
    }
}
