use async_graphql::MergedObject;

use self::{all_photos::AllPhotosQuery, total_photos::TotalPhotosQuery};

pub mod all_photos;
pub mod total_photos;

#[derive(MergedObject, Default)]
pub struct QueryRoot(TotalPhotosQuery, AllPhotosQuery);
