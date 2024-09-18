use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roast::Table)
                    .if_not_exists()
                    .col(pk_auto(Roast::Id))
                    .col(string(Roast::Level))
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Roast::Table)
            .columns([Roast::Level])
            .values_panic(["Light".into()])
            .values_panic(["Dark".into()])
            .values_panic(["Medium".into()])
            .values_panic(["Medium-Dark".into()])
            .values_panic(["Extra-Dark".into()])
            .values_panic(["Extra-light".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roast::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Roast {
    Table,
    Id,
    Level,
}
