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
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use tokio::net::TcpListener;

struct Query;

#[Object]
impl Query {
    async fn total_photos<'ctx>(&self, ctx: &Context<'ctx>) -> i64 {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        let counts = sqlx::query!("SELECT COUNT(*) as count FROM photos")
            .fetch_one(&**connection)
            .await
            .unwrap();
        counts.count.unwrap()
    }
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool.clone())
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
