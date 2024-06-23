use std::sync::Arc;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::{
    graphql::schema::root::{MutationRoot, QueryRoot},
    AppState,
};

pub async fn graphql_handler(
    State(state): State<Arc<AppState>>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(state.pool.clone())
    .finish();
    schema.execute(request.into_inner().data("aa")).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
