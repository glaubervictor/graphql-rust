use entity::dtos::person_dto::{PersonDto, PersonInputDto};
use entity::person;
pub use entity::person::Entity as PersonEntity;
use sea_orm::ActiveValue::Set;
use sea_orm::{error::DbErr, ActiveModelTrait, DatabaseConnection, EntityTrait, Order, QueryOrder};

pub struct PersonRepository;

impl PersonRepository {
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<PersonDto>, DbErr> {
        let persons = PersonEntity::find()
            .order_by(person::Column::Name, Order::Asc)
            .all(db)
            .await?;

        let persons_dtos = persons.iter().map(|person| person.into()).collect();

        Ok(persons_dtos)
    }

    pub async fn add_person(
        db: &DatabaseConnection,
        input: &PersonInputDto,
        user_id: &str) -> Result<PersonDto, async_graphql::Error> {
        PersonInputDto::validate(&input)?;

        let person_id = cuid2::create_id();

        let active_model = person::ActiveModel {
            id: Set(person_id.clone()),
            user_id: Set(user_id.to_string()),
            name: Set(input.name.clone()),
            cellphone: Set(input.cellphone.clone()),
            ..Default::default()
        };

        let result = active_model
            .insert(db)
            .await
            .map_err(|err| async_graphql::Error::new(format!("Erro ao inserir no banco: {}", err)))?;

        Ok(PersonDto {
            id: result.id,
            name: result.name,
            cellphone: result.cellphone,
        })
    }
}
