use async_graphql::{Context, Object};
use sqlx::{Pool, Postgres};

use crate::entities::photo::Photo;

#[derive(Default)]
pub struct PostPhotoMutation;

#[Object]
impl PostPhotoMutation {
    #[tracing::instrument(skip(self, ctx))]
    async fn post_photo<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
        description: String,
    ) -> Photo {
        let connection = &mut ctx.data_unchecked::<Pool<Postgres>>();
        let rng = rand::random::<u64>();
        let url = format!("{}{}", "https://aaaaa.s3/", rng); // S3にアップしてURL取得てきな動きをするはず
        let user_id = 1; // ctxからアクセストークンを取得してuser_idをゲットするのかな？
        sqlx::query_as!(
            Photo,
            r#"
            INSERT INTO photos (name, url, description, posted_by_user_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, url, description
        "#,
            name,
            url,
            description,
            user_id
        )
        .fetch_one(*connection)
        .await
        .unwrap()
    }
}
