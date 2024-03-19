use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::{Pool, Postgres};

use super::photo::Photo;

#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    country: String,
}

#[ComplexObject]
impl User {
    pub async fn posted_photos<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Photo> {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as(
            r#"
        SELECT photos.id, photos.name, url, description
        JOIN users on photos.posted_by_user_id = users.id
        WHERE users.id = $1
        "#,
        )
        .bind(self.id)
        .fetch_all(*connection)
        .await
        .unwrap()
    }
}
