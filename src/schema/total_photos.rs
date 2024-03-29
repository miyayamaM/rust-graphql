use async_graphql::{Context, Object};

use sqlx::{Pool, Postgres};

#[derive(Default)]

pub struct TotalPhotosQuery;

#[Object]
impl TotalPhotosQuery {
    async fn total_photos<'ctx>(&self, ctx: &Context<'ctx>) -> i64 {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        let counts = sqlx::query!("SELECT COUNT(*) as count FROM photos")
            .fetch_one(*connection)
            .await
            .unwrap();
        counts.count.unwrap()
    }
}
