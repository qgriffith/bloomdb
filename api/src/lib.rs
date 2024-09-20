use axum::http::StatusCode;

mod brewers;
mod roasts;
pub mod server;
mod users;

/// Converts an internal error into a tuple containing an HTTP status code and an error message.
///
/// # Arguments
///
/// * `err` - The error that occurred. It must implement the `std::error::Error` trait.
///
/// # Returns
///
/// A tuple containing:
/// * `StatusCode::INTERNAL_SERVER_ERROR` - Represents the 500 Internal Server Error HTTP status code.
/// * `String` - The string representation of the error.
pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
