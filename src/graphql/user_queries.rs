use crate::repository::user_repository::{User, UserRepository};
use async_graphql::{Context, Object};
use sea_orm::DatabaseConnection;
use entity::dtos::user_dto::UserDto;
use crate::auth::guards::RoleGuard;
use crate::auth::permissions::Role;

#[derive(Default)]
pub struct UserQueries;

#[Object]
impl UserQueries {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn users(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<UserDto>> {
        let db = ctx.data::<DatabaseConnection>()?;
        UserRepository::find_all(db).await.map_err(|e| e.into())
    }
}
