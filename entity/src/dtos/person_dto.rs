use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, SimpleObject)]
pub struct PersonDto {
    pub id: String,
    pub name: String,
    pub cellphone: String,
}