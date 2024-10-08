use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    Form,
};
use entity::recipe as Recipe;

use super::internal_error;
use sea_orm::{entity::*, query::*, DatabaseConnection};
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

/// Asynchronously fetches a recipe based on a provided slug.
///
/// # Arguments
///
/// * `State(conn)` - An instance of `State` holding a `DatabaseConnection`.
///   It is used to query the database for the recipe.
/// * `Path(slug)` - A `Path` containing the slug of the recipe to retrieve.
///
/// # Returns
///
/// * `Ok(Json(Some(recipe)))` - If a recipe matching the slug exists.
/// * `Ok(Json(None))` - If no recipe matches the provided slug.
/// * `Err((StatusCode, String))` - If there is an error fetching the recipe.
///
/// # Errors
///
/// Returns a tuple `(StatusCode, String)` wrapped in an `Err` variant if:
///
/// * There is an internal error when querying the database.
///
pub async fn get_recipe_slug(
    State(conn): State<DatabaseConnection>,
    Path(slug): Path<String>,
) -> Result<Json<Option<Recipe::Model>>, (StatusCode, String)> {
    let recipe = Recipe::Entity::find()
        .filter(Recipe::Column::Slug.eq(slug))
        .one(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipe))
}

/// Fetches recipes with a specific title from the database.
///
/// This asynchronous function retrieves all recipes that match the given title
/// from the connected database and returns them as JSON.
///
/// # Parameters
///
/// - `State(conn)`: A `State` wrapper around the `DatabaseConnection` required to access the database.
/// - `Path(title)`: A `Path` wrapper containing the title of the recipes to search for.
///
/// # Returns
///
/// - `Ok(Json<Vec<Recipe::Model>>)` on success, containing a JSON array of recipe models.
/// - `Err((StatusCode, String))` on failure, containing an HTTP status code and an error message.
///
/// # Errors
///
/// Returns an internal server error if the database query fails for any reason.
///
pub async fn get_recipe_title(
    State(conn): State<DatabaseConnection>,
    Path(title): Path<String>,
) -> Result<Json<Vec<Recipe::Model>>, (StatusCode, String)> {
    let recipes = Recipe::Entity::find()
        .filter(Recipe::Column::Title.contains(title))
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipes))
}

/// Asynchronous function to retrieve recipes for a specific roaster from the database.
///
/// # Parameters
///
/// - `State(conn)`: Represents the database connection state.
/// - `Path(roaster)`: The path parameter representing the roaster's name.
///
/// # Returns
///
/// - `Result<Json<Vec<Recipe::Model>>, (StatusCode, String)>`: The function returns a `Result` containing either:
///   - A JSON array of `Recipe::Model` on success.
///   - A tuple `(StatusCode, String)` on failure.
///
/// # Detailed Description
///
/// This function retrieves all recipes associated with a specific roaster from the database.
/// It filters the `Recipe` entities based on the `roaster` name provided in the path parameter.
/// The results are returned as a JSON array of `Recipe::Model`. If an error occurs during the database query,
/// it maps the error to an internal server error and returns it.
///
pub async fn get_recipes_roaster(
    State(conn): State<DatabaseConnection>,
    Path(roaster): Path<String>,
) -> Result<Json<Vec<Recipe::Model>>, (StatusCode, String)> {
    let recipes = Recipe::Entity::find()
        .filter(Recipe::Column::Roaster.eq(roaster))
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipes))
}

/// Fetches all recipes associated with a specified machine.
///
/// This asynchronous function takes a machine name as input and
/// queries the database to retrieve all recipes linked to that machine.
///
/// # Arguments
///
/// * `State(conn)` - The database connection state.
/// * `Path(machine)` - The machine name provided as a path parameter.
///
/// # Returns
///
/// This function returns a `Result`:
/// * `Ok(Json<Vec<Recipe::Model>>)` - A JSON response containing a vector of recipes.
/// * `Err((StatusCode, String))` - An error tuple containing the status code and error message.
///
/// # Errors
///
/// Returns an error if the database query fails.
/// The error is wrapped in an internal error function to provide a consistent error response.
///
pub async fn get_recipes_machine(
    State(conn): State<DatabaseConnection>,
    Path(machine): Path<String>,
) -> Result<Json<Vec<Recipe::Model>>, (StatusCode, String)> {
    let recipes = Recipe::Entity::find()
        .filter(Recipe::Column::Machine.eq(machine))
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(recipes))
}

/// Asynchronously creates a new recipe in the database.
///
/// # Arguments
///
/// * `State(conn): State<DatabaseConnection>`
///   - A managed state holding a [`DatabaseConnection`] used to perform database operations.
///
/// * `form: Form<Recipe::Model>`
///   - A form containing data for the recipe to be created.
///
/// # Returns
///
/// * `Result<Json<Option<Recipe::Model>>, (StatusCode, String)>`
///   - On success, it returns a JSON response with the created recipe model.
///   - On failure, it returns a tuple of `StatusCode` and an error message.
///
/// # Errors
///
/// This function can return an error if the insertion into the database fails.
/// The error will be encapsulated in a tuple containing the HTTP status code and an error message string.

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
        oauth_user: ActiveValue::set(form.oauth_user),
        user_id: ActiveValue::set(form.user_id),
        brewer_id: ActiveValue::set(form.brewer_id),
        roast_id: ActiveValue::set(form.roast_id),
        created_at: ActiveValue::set(form.created_at),
    };

    let result = recipe.insert(&conn).await.map_err(internal_error)?;
    Ok(Json(Some(result)))
}
