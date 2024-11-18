use crate::auth::jwt::create_token;
use crate::auth::permissions::Role;
use crate::repository::user_repository::UserRepository;
use async_graphql::{Context, FieldResult, Object};
use std::str::FromStr;
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct UserMutations {}

#[Object]
impl UserMutations {
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> FieldResult<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        match UserRepository::validate_user(db, &email, &password).await {
            Ok(user) => {
                let role = Role::from_str(&user.role)?;
                let token = create_token(&user.id, role)?;

                Ok(token)
            }
            Err(_) => {
                Err(async_graphql::Error::new("Credenciais inválidas"))
            }
        }
    }

    async fn register(&self, ctx: &Context<'_>, email: String, password: String) -> FieldResult<String> {
        let db = ctx.data::<DatabaseConnection>()?;
        match UserRepository::register_user(db, &email, &password).await {
            Ok(user) => {
                let role = Role::from_str(&user.role)?;
                let token = create_token(&user.id, role)?;

                Ok(token)
            }
            Err(_) => {
                Err(async_graphql::Error::new("Não foi possível registrar o usuário"))
            }
        }
    }
}
