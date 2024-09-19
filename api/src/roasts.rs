use axum::{extract::State, http::StatusCode, response::Json};
use entity::roast as Roast;

use sea_orm::DatabaseConnection;
use sea_orm::{entity::*, error::*, query::*, DbConn, FromQueryResult};

pub async fn get_roasts(
    State(conn): State<DatabaseConnection>,
) -> Result<Json<Vec<Roast::Model>>, (StatusCode, String)> {
    let roasts: Vec<Roast::Model> = Roast::Entity::find()
        .all(&conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(roasts))
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
