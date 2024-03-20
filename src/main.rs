use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    *,
};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use dotenv::dotenv;
use schema::{MutationRoot, QueryRoot};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tokio::net::TcpListener;

mod entities;
mod schema;

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let pool = establish_db_connection().await;
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(pool)
    .finish();

    // "/" でリクエストを待つ
    let app = Router::new().route(
        "/",
        get(graphql_playground).post_service(GraphQL::new(schema)),
    );

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
