use crate::auth::guards::RoleGuard;
use crate::auth::permissions::Role;
use crate::repository::person_repository::PersonRepository;
use async_graphql::{Context, Object};
use entity::dtos::person_dto::PersonDto;
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct PersonQueries;

#[Object]
impl PersonQueries {
    #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    async fn persons(&self, ctx: &Context<'_>) -> async_graphql::Result<Vec<PersonDto>> {
        let db = ctx.data::<DatabaseConnection>()?;
        PersonRepository::find_all(db).await.map_err(|e| e.into())
    }
}
