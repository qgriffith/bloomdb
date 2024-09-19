mod roasts;

use axum::{response::Html, routing::get, Router};

use sea_orm::{Database, DatabaseConnection};
use std::env;
use tokio::io::AsyncSeek;
use tower_cookies::{CookieManagerLayer, Cookies};

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
        .route("/roast/", get(roasts::get_roasts))
        .layer(CookieManagerLayer::new())
        .with_state(conn);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
