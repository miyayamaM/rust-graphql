use async_graphql::MergedObject;

use super::{
    all_photos::AllPhotosQuery, post_photo::PostPhotoMutation, total_photos::TotalPhotosQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TotalPhotosQuery, AllPhotosQuery);
#[derive(MergedObject, Default)]
pub struct MutationRoot(PostPhotoMutation);
