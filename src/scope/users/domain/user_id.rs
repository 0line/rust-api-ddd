use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::{Uuid};

#[derive(Debug)]
pub enum IdError {
    Empty,
    InvalidFormat(String),
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdError::Empty => write!(f, "Input id is required."),
            IdError::InvalidFormat(value) => write!(f, "The value <{}> is invalid", value),
        }
    }
}

impl std::error::Error for IdError {}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserId {
    #[serde(with = "uuid_serde")]
    value: Uuid,
}

impl UserId {
    pub fn new(value: String) -> Result<Self, IdError> {
        if value.is_empty() {
            return Err(IdError::Empty);
        }
        if !Uuid::parse_str(&value).is_ok() {
            return Err(IdError::InvalidFormat(value));
        }
        Ok(
            Self{
               value: Uuid::parse_str(&value).unwrap()
            }
        )
    }

    pub fn get_value(&self) -> &Uuid {
        &self.value
    }
}

// Implementación de la serialización y deserialización para `Uuid`
mod uuid_serde {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use uuid::Uuid;

    pub fn serialize<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&uuid.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uuid::parse_str(&s).map_err(serde::de::Error::custom)
    }
}
