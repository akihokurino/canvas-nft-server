use crate::ddb::Dao;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

const TABLE_NAME: &str = "canvas-nft-thumbnail";
const KEY_ID: &str = "ID";
const KEY_WORK_ID: &str = "WorkID";
const KEY_IMAGE_PATH: &str = "ImagePath";
const KEY_ORDER: &str = "Order";

impl work::Thumbnail {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (
            Some(AttributeValue::S(id)),
            Some(AttributeValue::S(work_id)),
            Some(AttributeValue::S(image_path)),
            Some(AttributeValue::N(order)),
        ) = (
            data.get(KEY_ID),
            data.get(KEY_WORK_ID),
            data.get(KEY_IMAGE_PATH),
            data.get(KEY_ORDER),
        ) {
            return Some(work::Thumbnail {
                id: id.to_owned(),
                work_id: work_id.to_owned(),
                image_path: image_path.to_owned(),
                order: order.to_owned().parse().unwrap(),
            });
        }
        None
    }

    async fn serialize_and_save(&self, cli: &Client, table_name: String) -> AppResult<()> {
        cli.put_item()
            .table_name(table_name)
            .item(KEY_ID, AttributeValue::S(self.id.to_owned()))
            .item(KEY_WORK_ID, AttributeValue::S(self.work_id.to_owned()))
            .item(
                KEY_IMAGE_PATH,
                AttributeValue::S(self.image_path.to_owned()),
            )
            .item(
                KEY_ORDER,
                AttributeValue::N(self.order.to_owned().to_string()),
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

impl Dao<work::Thumbnail> {
    pub async fn get_by_work(&self, work_id: String) -> AppResult<Vec<work::Thumbnail>> {
        let res = self
            .cli
            .query()
            .index_name("WorkID-Order-Index")
            .key_condition_expression("#key = :value".to_string())
            .expression_attribute_names("#key".to_string(), KEY_WORK_ID)
            .expression_attribute_values(
                ":value".to_string(),
                AttributeValue::S(work_id.to_owned()),
            )
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .send()
            .await?;

        let mut entities: Vec<work::Thumbnail> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(work::Thumbnail::deserialize(item).unwrap())
        }

        Ok(entities)
    }

    pub async fn put(&self, item: &work::Thumbnail) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.cli
            .delete_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_ID, work::Thumbnail::primary_key(id))
            .send()
            .await?;
        Ok(())
    }
}
