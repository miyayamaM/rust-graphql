use async_graphql::{ComplexObject, Context, SimpleObject};
use sqlx::{Pool, Postgres};

use super::user::User;

#[derive(SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
pub struct Photo {
    id: i32,
    name: String,
    url: String,
    description: String,
}

#[ComplexObject]
impl Photo {
    pub async fn posted_by<'ctx>(&self, ctx: &Context<'ctx>) -> User {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        sqlx::query_as(
            r#"
            SELECT users.id, users.name, email, country
            FROM users
            JOIN photos on photos.posted_by_user_id = users.id
            WHERE photos.id = $1
        "#,
        )
        .bind(self.id)
        .fetch_one(*connection)
        .await
        .unwrap()
    }
}
