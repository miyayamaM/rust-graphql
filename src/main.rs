use std::sync::Arc;

use crate::graphql::graphql_handler::graphql;
use axum::{routing::get, Router};
use database::connection::establish_db_connection;
use graphql::graphql_handler::graphql_playground;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::prelude::*;

mod database;
mod entities;
mod graphql;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();
    let pool = establish_db_connection().await;

    let state = Arc::new(AppState { pool });

    // "/" でリクエストを待つ
    let app = Router::new()
        .route("/graphql", get(graphql_playground).post(graphql))
        .layer(
            TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .with_state(state);

    // server を起動
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}
