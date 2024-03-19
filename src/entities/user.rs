use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::{Pool, Postgres};

use super::photo::Photo;

#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
pub struct User {
    user_id: i32,
    user_name: String,
    email: String,
    country: String,
}

#[ComplexObject]
impl User {
    async fn posted_photos<'ctx>(&self, ctx: &Context<'ctx>) -> Vec<Photo> {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as(
            r#"
        SELECT photos.id, photos.name, url, description, users.id as user_id, users.name as user_name, email, country
        FROM photos
        JOIN users on photos.posted_by_user_id = users.id
        WHERE users.id = $1
        "#
        )
        .bind(self.user_id)
        .fetch_all(*connection)
        .await
        .unwrap()
    }
}
