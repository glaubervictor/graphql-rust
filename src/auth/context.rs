use crate::auth::jwt::Claims;
use async_graphql::Context;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

pub fn get_authenticated_user(ctx: &Context<'_>) -> Option<AuthenticatedUser> {
    ctx.data_opt::<AuthenticatedUser>().cloned()
}
