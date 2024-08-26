use std::iter::Successors;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;
use std::option::Option;

#[derive(serde::Serialize, serde::Deserialize,Debug)]
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
    pub fn new(success: bool, message: Option<String>, data: Option<Vec<serde_json::Value>> , error:  Option<Vec<serde_json::Value>> ) -> Self {
        Self {
            success,
            message,
            data,
            error,
        }
    }
 /*pub fn display(mut self) -> Self {
        match self.message {
            Some(message) => self.message = Option::from(message.to_string()),
            None  => self.message = Option::from("".to_string())
        }
        match  self.data {
            Some(data) => self.data = Option::from(data.to_string()),
            None  => self.message = Option::from("".to_string())
        }
        match self.error {
            Some(error) => self.error = Option::from(error.to_string()),
            None  => self.error = Option::from("".to_string())
        }

    }*/
}