use async_graphql::{Context, Object, SimpleObject};
use sqlx::{Pool, Postgres};

#[derive(SimpleObject, sqlx::FromRow)]
struct User {
    user_id: i32,
    user_name: String,
    email: String,
    country: String,
}

#[derive(SimpleObject, sqlx::FromRow)]
struct Photo {
    id: i32,
    name: String,
    url: String,
    description: String,
    #[sqlx(flatten)]
    posted_by: User,
}

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
