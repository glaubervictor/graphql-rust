use sea_orm::{error::DbErr, DatabaseConnection, EntityTrait, Order, QueryOrder};
use entity::dtos::person_dto::PersonDto;
pub use entity::person::{Entity as PersonEntity, Model as Person};

pub struct PersonRepository;

impl PersonRepository {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<PersonDto>, DbErr> {
        let persons = PersonEntity::find()
            .order_by(entity::person::Column::Name, Order::Asc)
            .all(db)
            .await?;

        let persons_dtos = persons
            .iter().map(|person| person.into())
            .collect();

        Ok(persons_dtos)
    }
}
