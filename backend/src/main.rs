mod config;
mod db;
mod dtos;
mod errors;
mod handlers;
mod models;
mod routes;
mod utils;

use axum::http::{
    Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use dotenv::dotenv;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use tracing::level_filters::LevelFilter;

use crate::{config::Config, db::DBClient, routes::create_router};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    let config = Config::init();

    // Connectting to DB
    let db_options = SqliteConnectOptions::from_str(&config.database_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(db_options)
        .await?;

    println!("Connected to SQLite DB!");

    // Running Migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]);

    let db_client = DBClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    // Server Setup
    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());

    let addr = SocketAddr::from_str(&format!("{}:{}", &config.host, &config.port))?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("\nServer listening on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
