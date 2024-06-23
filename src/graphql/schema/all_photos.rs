use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};
use tracing::Instrument;

use crate::entities::photo::Photo;

#[derive(Default)]
pub struct AllPhotosQuery;

#[Object]
impl AllPhotosQuery {
    #[tracing::instrument(skip(self, ctx))]
    async fn all_photos<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Photo> {
        let query_span = tracing::info_span!("Getting all photos from the datanase.");
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as(
            r#"
        SELECT id, name, url, description
        FROM photos
        "#,
        )
        .fetch_all(*connection)
        .instrument(query_span)
        .await
        .unwrap()
    }
}
