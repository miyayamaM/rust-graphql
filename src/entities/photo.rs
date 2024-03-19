use async_graphql::SimpleObject;

use super::user::User;

#[derive(SimpleObject, sqlx::FromRow)]
pub struct Photo {
    id: i32,
    name: String,
    url: String,
    description: String,
    #[sqlx(flatten)]
    posted_by: User,
}
