use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{Result, Value};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct APIResponseSuccess {
    pub success: bool,
    pub message: String,
    pub data: Vec<serde_json::Value>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct APIResponseError {
    pub success: bool,
    pub message: String,
    pub error: Vec<serde_json::Value>,
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
