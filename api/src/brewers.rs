use super::internal_error;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use entity::brewer as Brewer;
use sea_orm::{entity::*, DatabaseConnection};

/// Retrieves a list of brewers from the database and returns them as a JSON response.
///
/// # Parameters:
/// - `State(conn)`: Extracted state which contains the `DatabaseConnection` to be used for database queries.
///
/// # Returns:
/// - `Result<Json<Vec<Brewer::Model>>, (StatusCode, String)>`:
///   - On success: a `Json` response containing a vector of `Brewer::Model`.
///   - On failure: a tuple containing an HTTP status code and a string message describing the error.
///
/// # Errors:
/// This function will return an error if the database query fails. The error will be converted
/// into an internal server error represented by an HTTP status code of 500 and an error message.
///
pub async fn get_brewers(
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Brewer::Model>>, (StatusCode, String)> {
    let brewers = Brewer::Entity::find()
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(brewers))
}

/// Fetches a Brewer entity by its ID from the database.
///
/// # Arguments
///
/// * `State(conn)` - A `State` wrapper containing a `DatabaseConnection` object. This provides the database connection to perform queries.
/// * `Path(id)` - A `Path` wrapper containing an `i32` representing the ID of the Brewer entity to be fetched.
///
/// # Returns
///
/// This function returns a `Result`:
/// - `Ok(Json<Option<Brewer::Model>>)` - If the entity is found successfully, it returns a `Json` wrapper containing an `Option` of `Brewer::Model`.
/// - `Err((StatusCode, String))` - If there is an error, it returns a tuple containing a `StatusCode` and a `String` with the error message.
///
/// # Errors
///
/// This function will return an error in the following cases:
/// - If there is an issue with the database connection or query execution, it will return an internal server error represented by a tuple containing an appropriate `StatusCode` and an error message.
pub async fn get_brewer_id(
    State(conn): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<Option<Brewer::Model>>, (StatusCode, String)> {
    let brewer = Brewer::Entity::find_by_id(id)
        .one(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(brewer))
}
