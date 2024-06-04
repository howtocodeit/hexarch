/*!
    Module `http` exposes an HTTP server that handles HTTP requests to the application. Its
    implementation is opaque to module consumers.
*/

use std::sync::Arc;

use anyhow::Context;
use axum::{http, Json, Router};
use axum::extract::State;
use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{post, Route};
use serde::{Deserialize, Serialize};
use tokio::net;
use tower_http::trace::{HttpMakeClassifier, TraceLayer};
use tower_layer::Layer;
use tracing::{instrument, Span};

use crate::domain::posts::models::author::{Author, CreateAuthorRequest};
use crate::domain::posts::models::errors::CreateAuthorError;
use crate::domain::posts::ports::{PostRepository, PostService};
use crate::inbound::http::handlers::create_author::create_author;
use crate::inbound::http::responses::{ErrorResponseBody, ErrorResponseData, ResponseBody};

mod handlers;
mod responses;

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
struct AppState<PS: PostService> {
    post_service: PS,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        post_service: impl PostService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        // Construct dependencies to inject into handlers.
        let state = Arc::new(AppState { post_service });

        let router = axum::Router::new()
            .route("/", axum::routing::get(root))
            .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes() -> Router<_> {
    Router::new().route("/authors", post(create_author))
}

async fn root() -> &'static str {
    "Hello, World!\n"
}
