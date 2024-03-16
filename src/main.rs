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
use tokio::net::TcpListener;

struct Query;

#[async_graphql::Object]
impl Query {
    async fn total_photos(&self) -> usize {
        42
    }
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

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
