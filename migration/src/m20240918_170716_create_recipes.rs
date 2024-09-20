use chrono::Utc;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use super::m20240918_162751_create_users::User;
use super::m20240918_164031_create_roasts::Roast;
use super::m20240918_164352_create_brewers::Brewer;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(pk_auto(Recipe::Id))
                    .col(string(Recipe::Title))
                    .col(string(Recipe::Slug))
                    .col(string(Recipe::Roaster))
                    .col(string(Recipe::Temp))
                    .col(string(Recipe::Link))
                    .col(string(Recipe::ShopLink))
                    .col(string(Recipe::Machine))
                    .col(string(Recipe::Type))
                    .col(integer(Recipe::UserId))
                    .col(integer(Recipe::BrewerId))
                    .col(integer(Recipe::RoastId))
                    .col(date_time(Recipe::CreatedAt))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_recipe_user_id")
                            .from(Recipe::Table, Recipe::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_recipe_roast_id")
                            .from(Recipe::Table, Recipe::RoastId)
                            .to(Roast::Table, Roast::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_recipe_brewer_id")
                            .from(Recipe::Table, Recipe::BrewerId)
                            .to(Brewer::Table, Brewer::Id),
                    )
                    .to_owned(),
            )
            .await?;
        let dt = Utc::now();
        let insert = Query::insert()
            .into_table(Recipe::Table)
            .columns([
                Recipe::Title,
                Recipe::Slug,
                Recipe::Roaster,
                Recipe::Temp,
                Recipe::Link,
                Recipe::ShopLink,
                Recipe::RoastId,
                Recipe::Machine,
                Recipe::BrewerId,
                Recipe::Type,
                Recipe::UserId,
                Recipe::CreatedAt,
            ])
            .values_panic([
                "The Future".into(),
                "the-future".into(),
                "Black and White".into(),
                "hot".into(),
                "https://share-h5.xbloom.com/?id=8yAUAWJktyHNIpo3vjZ6pA==".into(),
                "https://xbloom.com/products/the-future-xbloom-exclusive".into(),
                1.into(),
                "Studio".into(),
                1.into(),
                "xbloom".into(),
                1.into(),
                dt.into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Recipe {
    Table,
    Id,
    Title,
    Slug,
    Roaster,
    Temp,
    Link,
    ShopLink,
    Machine,
    Type,
    UserId,
    BrewerId,
    RoastId,
    CreatedAt,
}
