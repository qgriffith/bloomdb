use chrono::Utc;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id))
                    .col(string(User::Email))
                    .col(string(User::Username))
                    .col(date_time(User::CreatedAt))
                    .to_owned(),
            )
            .await?;
        let dt = Utc::now();
        let insert = Query::insert()
            .into_table(User::Table)
            .columns([User::Email, User::Username, User::CreatedAt])
            .values_panic(["admin@localhost.com".into(), "admin".into(), dt.into()])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    Username,
    CreatedAt,
}
