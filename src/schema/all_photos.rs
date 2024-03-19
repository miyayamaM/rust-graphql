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
        SELECT photos.id, photos.name, url, description, users.id as user_id, users.name as user_name, email, country
        FROM photos
        JOIN users on photos.posted_by_user_id = users.id
        "#,
        )
        .fetch_all(*connection)
        .await
        .unwrap()
    }
}
