use crate::auth::guards::RoleGuard;
use crate::auth::permissions::Role;
use crate::repository::user_repository::UserRepository;
use async_graphql::{Context, Object};
use entity::dtos::user_dto::UserDto;
use sea_orm::DatabaseConnection;

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
