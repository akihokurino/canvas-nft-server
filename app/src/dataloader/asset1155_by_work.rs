use crate::ddb::Dao;
use crate::domain::*;
use crate::AppResult;
use async_trait::async_trait;
use dataloader::{cached, BatchFn};
use std::collections::HashMap;

pub struct Batcher {}

#[async_trait]
impl BatchFn<String, AppResult<asset::Asset1155>> for Batcher {
    async fn load(&mut self, keys: &[String]) -> HashMap<String, AppResult<asset::Asset1155>> {
        let mut hashmap = HashMap::new();
        let dao: Dao<asset::Asset1155> = Dao::new().await;
        for id in keys {
            let item = dao.get(id.clone()).await;
            hashmap.insert(id.to_owned(), item);
        }
        hashmap
    }
}

impl Batcher {
    pub fn new_loader() -> Loader {
        cached::Loader::new(Batcher {}).with_max_batch_size(100)
    }
}

pub type Loader = cached::Loader<String, AppResult<asset::Asset1155>, Batcher>;
