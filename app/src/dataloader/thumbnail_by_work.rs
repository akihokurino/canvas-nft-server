use crate::ddb::Dao;
use crate::domain::*;
use crate::AppResult;
use async_trait::async_trait;
use dataloader::{cached, BatchFn};
use std::collections::HashMap;

pub struct Batcher {}

#[async_trait]
impl BatchFn<String, AppResult<Vec<work::Thumbnail>>> for Batcher {
    async fn load(&mut self, keys: &[String]) -> HashMap<String, AppResult<Vec<work::Thumbnail>>> {
        let mut hashmap = HashMap::new();
        let dao: Dao<work::Thumbnail> = Dao::new().await;
        for id in keys {
            let items = dao.get_by_work(id.clone()).await;
            hashmap.insert(id.to_owned(), items);
        }
        hashmap
    }
}

impl Batcher {
    pub fn new_loader() -> Loader {
        cached::Loader::new(Batcher {}).with_max_batch_size(100)
    }
}

pub type Loader = cached::Loader<String, AppResult<Vec<work::Thumbnail>>, Batcher>;
