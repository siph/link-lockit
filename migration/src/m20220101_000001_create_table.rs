use sea_schema::migration::{prelude::*, sea_orm::DbBackend};
use sea_orm::Schema;
use entity::links::Entity as links;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(DbBackend::Postgres);
        manager.create_table(schema.create_table_from_entity(links)).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(links).to_owned())
            .await?;
        Ok(())
    }
}
