use super::m20240918_170716_create_recipes;
use crate::sea_orm::sqlx::query_as_with;
use sea_orm_migration::{prelude::*, schema::*};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(m20240918_170716_create_recipes::Recipe::Table)
                    .add_column_if_not_exists(ColumnDef::new(Recipe::OauthUser).string())
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(m20240918_170716_create_recipes::Recipe::Table)
                    .drop_column(Recipe::OauthUser)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Recipe {
    Table,
    OauthUser,
}
