use std::fmt;
use std::str::FromStr;

pub enum Role {
    Admin,
    User,
}

impl Role {
    // Função para verificar se o papel corresponde ao autorizado
    pub fn is_authorized(&self, user_role: &str) -> bool {
        match self {
            Role::Admin => user_role == "Admin" || user_role == "User", //Admin pode acessar como usuário
            Role::User => user_role == "User"
        }
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Admin" => Ok(Role::Admin),
            "User" => Ok(Role::User),
            _ => Err(format!("Invalid role: {}", s)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
        }
    }
}