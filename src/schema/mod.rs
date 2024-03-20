use async_graphql::MergedObject;

use self::{
    all_photos::AllPhotosQuery, post_photo::PostPhotoMutation, total_photos::TotalPhotosQuery,
};

pub mod all_photos;
pub mod post_photo;
pub mod total_photos;

#[derive(MergedObject, Default)]
pub struct QueryRoot(TotalPhotosQuery, AllPhotosQuery);
#[derive(MergedObject, Default)]
pub struct MutationRoot(PostPhotoMutation);
