use axum::response::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Form,
};
use entity::recipe as Recipe;

use super::internal_error;
use sea_orm::{entity::*, DatabaseConnection};
use slug::slugify;

/// Asynchronously retrieves a list of recipes from the database.
///
/// This function uses the provided `DatabaseConnection` to query for all recipes
/// and returns them as a JSON response. If an error occurs during the query,
/// it is mapped to an appropriate status code and error message.
///
/// # Arguments
///
/// * `State(conn)`: A state containing the `DatabaseConnection` needed to execute the query.
///
/// # Returns
///
/// This function returns a `Result` that, on success, contains a `Json` response
/// with a vector of `Recipe::Model`. On failure, it returns a tuple containing
/// a `StatusCode` and an error message.
///
/// # Errors
///
/// This function might return the following errors:
///
/// * `internal_error`: If there is an issue querying the database, the error is
///   mapped to an internal server error (`StatusCode::INTERNAL_SERVER_ERROR`).
///
pub async fn get_recipes(
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Recipe::Model>>, (StatusCode, String)> {
    let recipes = Recipe::Entity::find()
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipes))
}

/// Fetches a recipe by its ID from the database.
///
/// # Arguments
///
/// - `State(conn)`: A shared state containing a database connection.
/// - `Path(id)`: The ID of the recipe to fetch, extracted from the request path.
///
/// # Returns
///
/// - `Result<Json<Option<Recipe::Model>>, (StatusCode, String)>`:
///   - On success: A JSON object containing the recipe, wrapped in an `Option`.
///       - If the recipe exists, returns `Some(recipe)`.
///       - If the recipe does not exist, returns `None`.
///   - On failure: An error tuple containing an HTTP status code and an error message.
///
/// # Errors
///
/// This function will return an error tuple (`(StatusCode, String)`) in the following scenarios:
/// - Database connection issues.
/// - Internal server errors during query execution.
///
pub async fn get_recipe_id(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Recipe::Model>>, (StatusCode, String)> {
    let recipe = Recipe::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipe))
}
pub async fn create_recipe(
    State(conn): State<DatabaseConnection>,
    form: Form<Recipe::Model>,
) -> Result<Json<Option<Recipe::Model>>, (StatusCode, String)> {
    let form = form.0;
    let slug = slugify(&form.title);
    let recipe = Recipe::ActiveModel {
        id: Default::default(),
        title: ActiveValue::set(form.title),
        slug: ActiveValue::set(slug),
        roaster: ActiveValue::set(form.roaster),
        temp: ActiveValue::set(form.temp),
        link: ActiveValue::set(form.link),
        shop_link: ActiveValue::set(form.shop_link),
        machine: ActiveValue::set(form.machine),
        creator: ActiveValue::set(form.creator),
        user_id: ActiveValue::set(form.user_id),
        brewer_id: ActiveValue::set(form.brewer_id),
        roast_id: ActiveValue::set(form.roast_id),
        created_at: ActiveValue::set(form.created_at),
    };

    let result = recipe.insert(&conn).await.map_err(internal_error)?;
    Ok(Json(Some(result)))
}
