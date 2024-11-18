use crate::utils::graphql_validation::IntoGraphQLError;
use async_graphql::{InputObject, SimpleObject};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, SimpleObject)]
#[graphql(name = "Person")]
pub struct PersonDto {
    pub id: String,
    pub name: String,
    pub cellphone: String,
}

#[derive(Clone, Debug, PartialEq, Validate, InputObject)]
#[graphql(name = "PersonInput")]
pub struct PersonInputDto {
    #[validate(length(min = 3, message = "O nome precisa ter pelo menos 3 caracteres"))]
    pub name: String,

    #[validate(length(
        min = 11,
        max = 11,
        message = "O número de celular deve conter exatamente 11 dígitos"
    ))]
    pub cellphone: String,
}

impl PersonInputDto {
    pub fn validate(dto: &PersonInputDto) -> Result<(), async_graphql::Error> {
        match dto.validate() {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into_graphql_error()),
        }
    }
}
