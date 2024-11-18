use entity::dtos::user_dto::UserDto;
pub use entity::user::{Entity as UserEntity, Model as User};
use sea_orm::{
    error::DbErr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order,
    QueryFilter, QueryOrder, Set,
};

pub struct UserRepository;

impl UserRepository {
    pub async fn validate_user(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> Result<User, String> {
        let user = UserEntity::find()
            .filter(entity::user::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(|_| "Erro ao consultar usuário")?;

        if let Some(user) = user {
            if user.check_password(password) {
                Ok(user)
            } else {
                Err("Usuário e/ou senha incorretos".to_string())
            }
        } else {
            Err("Usuário e/ou senha incorretos".to_string())
        }
    }

    pub async fn add_user(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> Result<User, String> {
        let active_model = entity::user::ActiveModel {
            id: Set(cuid2::create_id()),
            email: Set(email.to_string()),
            password_hash: Set(User::hash_password(password)
                .map_err(|_| "Erro ao gerar hash da senha".to_string())?),
            role: Set("Admin".to_string()),
            ..Default::default()
        };

        match active_model.insert(db).await {
            Ok(model) => Ok(model),
            Err(err) => {
                eprintln!("Erro ao criar usuário: {:?}", err);
                Err("Erro ao criar usuário no banco de dados".to_string())
            }
        }
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<UserDto>, DbErr> {
        let users = UserEntity::find()
            .order_by(entity::user::Column::Email, Order::Asc)
            .all(db)
            .await?;

        let user_dtos: Vec<UserDto> = users.iter().map(|user| user.into()).collect();

        Ok(user_dtos)
    }
}
