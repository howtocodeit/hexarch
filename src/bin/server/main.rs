use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context;
use axum::routing::post;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use tokio::net;

use hexarch::config::Config;
use hexarch::routes::{AppState, create_author};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    // A minimal tracing middleware for request logging.
    tracing_subscriber::fmt::init();
    let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
        |request: &axum::extract::Request<_>| {
            let uri = request.uri().to_string();
            tracing::info_span!("http_request", method = ?request.method(), uri)
        },
    );

    let sqlite = SqlitePool::connect_with(
        SqliteConnectOptions::from_str(&config.database_url)
            .with_context(|| format!("invalid database path {}", &config.database_url))?
            .pragma("foreign_keys", "ON"),
    )
    .await
    .with_context(|| format!("failed to open database at {}", &config.database_url))?;

    let app_state = AppState {
        sqlite: Arc::new(sqlite),
    };
    let router = axum::Router::new()
        .route("/authors", post(create_author))
        .layer(trace_layer)
        .with_state(app_state);
    let listener = net::TcpListener::bind(format!("0.0.0.0:{}", &config.server_port))
        .await
        .with_context(|| format!("failed to listen on {}", &config.server_port))?;

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router)
        .await
        .context("received error from running server")?;

    Ok(())
}
