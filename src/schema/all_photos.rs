use async_graphql::{Context, Object, SimpleObject};
use sqlx::{Pool, Postgres};

#[derive(SimpleObject)]
struct Photo {
    id: i32,
    name: String,
    url: String,
    description: String,
}

#[derive(Default)]
pub struct AllPhotosQuery;

#[Object]
impl AllPhotosQuery {
    async fn all_photos<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Photo> {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as!(Photo, "SELECT id, name, url, description FROM photos")
            .fetch_all(*connection)
            .await
            .unwrap()
    }
}
