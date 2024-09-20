use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use entity::roast as Roast;

use super::internal_error;
use sea_orm::{entity::*, DatabaseConnection};

/// Retrieves a list of roasts from the database.
///
/// This asynchronous function interacts with a database connection provided via the `State`
/// wrapper and returns a JSON response containing a vector of `Roast::Model`. In case of an
/// error, it returns an appropriate `StatusCode` and error message.
///
/// # Parameters
///
/// - `State(conn)`: A state wrapper containing a `DatabaseConnection` which is used to query the database.
///
/// # Returns
///
/// - `Result<Json<Vec<Roast::Model>>, (StatusCode, String)>`:
///     - On success, it returns `Ok(Json(roasts))`, where `roasts` is a vector of `Roast::Model` instances retrieved from the database.
///     - On failure, it returns an error tuple containing a `StatusCode` and a descriptive error message.
///
/// # Errors
///
/// This function may return the following errors:
/// - `(StatusCode::INTERNAL_SERVER_ERROR, String::from("Error message"))` if there is an error while querying the database.

pub async fn get_roasts(
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Roast::Model>>, (StatusCode, String)> {
    let roasts = Roast::Entity::find()
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(roasts))
}

/// Asynchronous endpoint to get a roast by its ID.
///
/// # Arguments
///
/// * `State(conn)` - A state extractor that provides a `DatabaseConnection`.
/// * `Path(id)` - An extractor that provides the `id` parameter from the request path.
///
/// # Returns
///
/// * `Result<Json<Option<Roast::Model>>, (StatusCode, String)>` - A JSON response containing
///   either the `Roast` model if found, or `None` if not found. In case of an error, it returns
///   a tuple with an HTTP status code and an error message.
///
/// # Errors
///
/// This function can return an error in the form of `(StatusCode, String)` if:
///
/// * There is an issue interacting with the database.
/// * There is an internal error during the `find_by_id` operation.
///
pub async fn get_roast_id(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Roast::Model>>, (StatusCode, String)> {
    let roast = Roast::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(roast))
}
