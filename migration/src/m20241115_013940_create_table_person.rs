use crate::m20241114_233954_create_table_user::User;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let mut foreign_key = ForeignKey::create()
            .from(Person::Table, Person::UserId)
            .to(User::Table, User::Id)
            .on_delete(ForeignKeyAction::Restrict)
            .on_update(ForeignKeyAction::Restrict)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(pk_uuid(Person::Id).string_len(24).not_null())
                    .col(uuid(Person::UserId).string_len(24).not_null())
                    .foreign_key(&mut foreign_key)
                    .col(string(Person::Name).string_len(120).not_null())
                    .col(string(Person::Cellphone).string_len(20).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Person {
    Table,
    Id,
    UserId,
    Name,
    Cellphone,
}
