use async_graphql::SimpleObject;

#[derive(Clone, Debug, PartialEq, SimpleObject)]
#[graphql(name = "User")]
pub struct UserDto {
    pub email: String,
    pub role: String,
}