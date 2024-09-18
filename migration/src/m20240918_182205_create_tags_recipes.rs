use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;
use super::m20240918_170443_create_tags::Tag;
use super::m20240918_170716_create_recipes::Recipe;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TagRecipe::Table)
                    .if_not_exists()
                    .col(pk_auto(TagRecipe::Id))
                    .col(integer(TagRecipe::TagId))
                    .col(integer(TagRecipe::RecipeId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_tagrecipe_tag_id")
                            .from(TagRecipe::Table, TagRecipe::TagId)
                            .to(Tag::Table, Tag::Id)
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_tagrecipe_recipe_id")
                            .from(TagRecipe::Table, TagRecipe::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TagRecipe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TagRecipe {
    Table,
    Id,
    TagId,
    RecipeId
}
