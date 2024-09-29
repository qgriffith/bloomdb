pub use sea_orm_migration::prelude::*;

mod m20240918_162751_create_users;
mod m20240918_164031_create_roasts;
mod m20240918_164352_create_brewers;
mod m20240918_170443_create_tags;
mod m20240918_170716_create_recipes;
mod m20240918_182205_create_tags_recipes;
mod m20240929_184147_alter_recipe_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240918_162751_create_users::Migration),
            Box::new(m20240918_164031_create_roasts::Migration),
            Box::new(m20240918_164352_create_brewers::Migration),
            Box::new(m20240918_170443_create_tags::Migration),
            Box::new(m20240918_170716_create_recipes::Migration),
            Box::new(m20240918_182205_create_tags_recipes::Migration),
            Box::new(m20240929_184147_alter_recipe_user::Migration),
        ]
    }
}
