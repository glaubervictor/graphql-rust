use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Serialize, Deserialize};
use crate::auth::permissions::Role;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // ID do usuário
    pub exp: usize,  // Data de expiração
    pub role: String, // Papel do usuário
}

const SECRET: &str = "your_secret_key";

// Função para criar um token
pub fn create_token(user_id: &str, role: Role) -> Result<String, Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Erro ao calcular expiração")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
        role: role.to_string(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
}

// Função para decodificar e validar um token
pub fn decode_token(token: &str) -> Result<Claims, Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
