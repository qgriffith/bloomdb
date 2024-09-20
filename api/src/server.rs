use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use crate::roasts;
use axum::error_handling::HandleErrorLayer;
use sea_orm::Database;
use std::env;
use std::time::Duration;
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let app = Router::new()
        .route("/", get(handler))
        .route("/api/roast", get(roasts::get_roasts))
        .route("/api/roast/:id", get(roasts::get_roast_id))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(30))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(conn);

    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
