use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

use crate::entities::photo::Photo;

#[derive(Default)]
pub struct AllPhotosQuery;

#[Object]
impl AllPhotosQuery {
    async fn all_photos<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Photo> {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as(
            r#"
        SELECT id, name, url, description
        FROM photos
        "#,
        )
        .fetch_all(*connection)
        .await
        .unwrap()
    }
}
