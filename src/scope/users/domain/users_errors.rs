use std::fmt;

#[derive(Debug)]
pub enum UserError {
    Empty(String),
    Invalid(String),
    InvalidFormat(String, String),
    TooShort(String, u8),
}


impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::Empty(value) => write!(f, "Input {} is required.", value),
            UserError::Invalid(value) => write!(f, "The value {} is invalid", value),
            UserError::TooShort(value, size) => write!(f, "The {} value of is too short, it must contain at least {} characters.", value, size),
            UserError::InvalidFormat(value, format) => write!(f, "The input  <{}> must have the format {}", value, format),
        }
    }
}

impl std::error::Error for UserError {}