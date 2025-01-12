use std::convert::TryInto;
use crate::scope::users::domain::user::User;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub uuid: String, // uuid
    pub email: String,
    #[sea_orm(column_type = "Text")]
    pub pwd: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {

}

impl From<User> for Model {
    fn from(user: User) -> Self {
        Model {
            uuid: user.get_id().to_string(),
            email: user.get_email().to_string(),
            pwd: user.get_pwd().to_string(),
        }
    }
}
/*
impl TryInto<User> for Model {
    type Error = String;

    fn try_into(self) -> Result<User, Self::Error> {
        //Ok(User::new(User::get_id(), User::get_email(), User::get_pwd()))
       let U= User::save(self.uuid, self.email, self.pwd);
       Ok(U)
    }
}*/