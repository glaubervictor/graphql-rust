use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

impl Model {
    pub fn check_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }
}

impl From<&Model> for crate::dtos::user_dto::UserDto {
    fn from(user: &Model) -> Self {
        crate::dtos::user_dto::UserDto {
            email: user.email.clone(),
            role: user.role.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
