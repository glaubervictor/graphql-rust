use crate::auth::context::get_authenticated_user;
use crate::auth::permissions::Role;
use async_graphql::{Context, Guard, Result};

pub struct RoleGuard {
    role: Role,
}

impl RoleGuard {
    pub fn new(role: Role) -> Self {
        RoleGuard { role }
    }
}

impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        if let Some(user) = get_authenticated_user(ctx) {
            let user_role = &user.claims.role;
            if !self.role.is_authorized(user_role) {
                return Err(async_graphql::Error::new("Acesso negado: você não tem permissão para acessar este recurso"));
            }
        } else {
            return Err(async_graphql::Error::new("Acesso negado: usuário não autenticado"));
        }

        Ok(())
    }
}
