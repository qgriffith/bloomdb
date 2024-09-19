use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Brewer::Table)
                    .if_not_exists()
                    .col(pk_auto(Brewer::Id))
                    .col(string(Brewer::Type))
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Brewer::Table)
            .columns([Brewer::Type])
            .values_panic(["Omni Dripper v2".into()])
            .values_panic(["xPod".into()])
            .values_panic(["Aeropress".into()])
            .values_panic(["Omni Dripper v1".into()])
            .values_panic(["Other".into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Brewer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Brewer {
    Table,
    Id,
    Type,
}
