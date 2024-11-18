use crate::auth::context::AuthenticatedUser;
use crate::repository::person_repository::PersonRepository;
use async_graphql::{Context, FieldResult, Object};
use entity::dtos::person_dto::{PersonDto, PersonInputDto};
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct PersonMutations {}

#[Object]
impl PersonMutations {
    async fn add_person(&self, ctx: &Context<'_>, input: PersonInputDto) -> FieldResult<PersonDto> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = ctx.data::<AuthenticatedUser>()?;

        match PersonRepository::add_person(db, &input, &user.claims.sub).await {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }
}