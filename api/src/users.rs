use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

use super::internal_error;
use entity::user as User;
use sea_orm::{entity::*, DatabaseConnection, FromQueryResult, QuerySelect};
use serde::{Deserialize, Serialize};

/// Asynchronously retrieves a list of users from the database.
///
/// This function uses a given `DatabaseConnection` to fetch the list of users
/// with only the `Id` and `Username` columns. The result is then converted
/// into a JSON vector of `PartialUser` structures and returned.
///
/// # Arguments
///
/// - `State(conn)`: A `State` wrapper containing the database connection.
///
/// # Returns
///
/// - `Result<Json<Vec<PartialUser>>, (StatusCode, String)>`:
///   - `Ok(Json(Vec<PartialUser>))`: JSON vector of users if the retrieval is successful.
///   - `Err((StatusCode, String))`: A tuple containing an HTTP status code and an error
///     message if the retrieval fails.
///
/// # Errors
///
/// This function returns an error if there is an issue with querying the database.
pub async fn get_users(
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<PartialUser>>, (StatusCode, String)> {
    let users = User::Entity::find()
        .select_only()
        .columns([User::Column::Id, User::Column::Username])
        .into_model::<PartialUser>()
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(users))
}

/// Retrieves a user's partial information by their ID.
///
/// This asynchronous function handles a request to fetch a user's details based on
/// their ID. It returns a JSON response containing the user's ID and username.
///
/// # Arguments
///
/// * `State(conn)`: The database connection wrapped in a `State`.
/// * `Path(id)`: The user ID as a path parameter.
///
/// # Returns
///
/// This function returns a `Result<Json<Option<PartialUser>>, (StatusCode, String)>`.
/// - On success: `Ok(Json(Some(partial_user)))` containing the user's partial details wrapped in `Json`.
/// - On error: `Err((StatusCode, String))` with an appropriate status code and error message.
///
/// # Errors
///
/// This function maps any internal errors encountered during the database query to a custom
/// error using the `internal_error` mapping function.
pub async fn get_user_id(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<PartialUser>>, (StatusCode, String)> {
    let user = User::Entity::find_by_id(id)
        .select_only()
        .columns([User::Column::Id, User::Column::Username])
        .into_model::<PartialUser>()
        .one(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(user))
}

/// Restricts what is returned from the User model to prevent
/// sensitive info from appearing in the results.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult)]
pub struct PartialUser {
    pub id: i32,
    pub username: String,
}
