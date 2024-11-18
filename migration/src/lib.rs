pub use sea_orm_migration::prelude::*;

mod m20241114_233954_create_table_user;
mod m20241115_013940_create_table_person;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241114_233954_create_table_user::Migration),
            Box::new(m20241115_013940_create_table_person::Migration),
        ]
    }
}
