use axum::{response::{Html, IntoResponse}, routing::get, Router, http::StatusCode};

use sea_orm::Database;
use std::env;
use tower_cookies::{CookieManagerLayer, Cookies};
use crate::roasts;

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
        .layer(CookieManagerLayer::new())
        .with_state(conn);

    let app = app.fallback(handler_404);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());;
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
