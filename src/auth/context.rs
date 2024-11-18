use async_graphql::Context;
use crate::auth::jwt::Claims;

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub claims: Claims,
}

pub fn get_authenticated_user(ctx: &Context<'_>) -> Option<AuthenticatedUser> {
    ctx.data_opt::<AuthenticatedUser>().cloned()
}
