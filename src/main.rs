use crate::graphql::graphql_handler::graphql;
use async_graphql::*;
use axum::{routing::get, Router};
use dotenv::dotenv;
use graphql::graphql_handler::graphql_playground;
use schema::{MutationRoot, QueryRoot};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::prelude::*;

mod entities;
mod graphql;
mod schema;

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
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .finish();
    let state = AppState { schema };

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

async fn establish_db_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap()
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, MutationRoot, EmptySubscription>,
}
