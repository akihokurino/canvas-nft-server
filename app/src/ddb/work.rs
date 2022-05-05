use crate::ddb::{Dao, PagingKey};
use crate::domain::work::WorkStatus;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::model::*;
use aws_sdk_dynamodb::Client;
use std::collections::{HashMap, HashSet};

const TABLE_NAME: &str = "canvas-nft-work";
const KEY_ID: &str = "ID";
const KEY_VIDEO_PATH: &str = "VideoPath";
const KEY_STATUS: &str = "Status";

impl work::Work {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (
            Some(AttributeValue::S(id)),
            Some(AttributeValue::S(video_path)),
            Some(AttributeValue::S(status)),
        ) = (
            data.get(KEY_ID),
            data.get(KEY_VIDEO_PATH),
            data.get(KEY_STATUS),
        ) {
            let data = work::Work {
                id: id.to_owned(),
                video_path: video_path.to_owned(),
                status: WorkStatus::from(status.to_owned().to_string()),
            };

            return Some(data);
        }
        None
    }

    async fn serialize_and_save(&self, cli: &Client, table_name: String) -> AppResult<()> {
        cli.put_item()
            .table_name(table_name)
            .item(KEY_ID, AttributeValue::S(self.id.to_owned()))
            .item(
                KEY_VIDEO_PATH,
                AttributeValue::S(self.video_path.to_owned()),
            )
            .item(
                KEY_STATUS,
                AttributeValue::S(self.status.to_owned().to_string()),
            )
            .send()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    fn primary_key(id: String) -> AttributeValue {
        AttributeValue::S(id.to_owned())
    }
}

impl Dao<work::Work> {
    pub async fn get_with_pager(
        &self,
        key: PagingKey,
        limit: Option<i32>,
    ) -> AppResult<(Vec<work::Work>, PagingKey)> {
        let res = self
            .cli
            .scan()
            .set_exclusive_start_key(key.val)
            .limit(limit.unwrap_or(20))
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .send()
            .await?;

        let mut entities: Vec<work::Work> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(work::Work::deserialize(item).unwrap())
        }

        Ok((entities, PagingKey::from(res.last_evaluated_key)))
    }

    pub async fn get_by_status_with_pager(
        &self,
        status: work::WorkStatus,
        key: PagingKey,
        limit: Option<i32>,
    ) -> AppResult<(Vec<work::Work>, PagingKey)> {
        let res = self
            .cli
            .query()
            .index_name("Status-Index")
            .key_condition_expression("#key = :value".to_string())
            .expression_attribute_names("#key".to_string(), KEY_STATUS)
            .expression_attribute_values(
                ":value".to_string(),
                AttributeValue::S(status.to_string()),
            )
            .set_exclusive_start_key(key.val)
            .limit(limit.unwrap_or(20))
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .send()
            .await?;

        let mut entities: Vec<work::Work> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(work::Work::deserialize(item).unwrap())
        }

        Ok((entities, PagingKey::from(res.last_evaluated_key)))
    }

    pub async fn get_multi(&self, ids: Vec<String>) -> AppResult<Vec<work::Work>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let ids: HashSet<String> = ids.into_iter().collect();
        let mut builder = keys_and_attributes::Builder::default();
        for id in ids {
            let mut key: HashMap<String, AttributeValue> = HashMap::new();
            key.insert(KEY_ID.to_string(), AttributeValue::S(id.to_owned()));
            builder = builder.keys(key);
        }

        let res = self
            .cli
            .batch_get_item()
            .request_items(self.table_name_provider.with(TABLE_NAME), builder.build())
            .send()
            .await?;

        let mut entities: Vec<work::Work> = vec![];
        for (table, data) in res.responses.unwrap_or_default() {
            if table == self.table_name_provider.with(TABLE_NAME) {
                for item in data {
                    entities.push(work::Work::deserialize(item).unwrap())
                }
            }
        }

        Ok(entities)
    }

    pub async fn get(&self, id: String) -> AppResult<work::Work> {
        let res = self
            .cli
            .get_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_ID, work::Work::primary_key(id))
            .send()
            .await?;

        if res.item.is_none() {
            return Err(AppError::NotFound);
        }

        let data = res.item.unwrap();

        Ok(work::Work::deserialize(data).unwrap())
    }

    pub async fn put(&self, item: &work::Work) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.cli
            .delete_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_ID, work::Work::primary_key(id))
            .send()
            .await?;
        Ok(())
    }
}
