use crate::ddb::Dao;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

const TABLE_NAME: &str = "canvas-nft-asset1155";
const KEY_WORK_ID: &str = "WorkID";
const KEY_CONTRACT_ADDRESS: &str = "ContractAddress";
const KEY_TOKEN_ID: &str = "TokenID";
const KEY_NAME: &str = "Name";
const KEY_DESCRIPTION: &str = "Description";
const KEY_IMAGE_URL: &str = "ImageURL";
const KEY_IMAGE_PREVIEW_URL: &str = "ImagePreviewURL";
const KEY_PERMALINK: &str = "Permalink";
const KEY_USD_PRICE: &str = "UsdPrice";
const KEY_ETH_PRICE: &str = "EthPrice";

impl asset::Asset1155 {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (
            Some(AttributeValue::S(work_id)),
            Some(AttributeValue::S(address)),
            Some(AttributeValue::S(token_id)),
            Some(AttributeValue::S(name)),
            Some(AttributeValue::S(description)),
            Some(AttributeValue::S(image_url)),
            Some(AttributeValue::S(image_preview_url)),
            Some(AttributeValue::S(permalink)),
            Some(AttributeValue::N(usd_price)),
            Some(AttributeValue::N(eth_price)),
        ) = (
            data.get(KEY_WORK_ID),
            data.get(KEY_CONTRACT_ADDRESS),
            data.get(KEY_TOKEN_ID),
            data.get(KEY_NAME),
            data.get(KEY_DESCRIPTION),
            data.get(KEY_IMAGE_URL),
            data.get(KEY_IMAGE_PREVIEW_URL),
            data.get(KEY_PERMALINK),
            data.get(KEY_USD_PRICE),
            data.get(KEY_ETH_PRICE),
        ) {
            let data = asset::Asset1155 {
                work_id: work_id.to_owned(),
                contract_address: address.to_owned(),
                token_id: token_id.to_owned(),
                name: name.to_owned(),
                description: description.to_owned(),
                image_url: image_url.to_owned(),
                image_preview_url: image_preview_url.to_owned(),
                permalink: permalink.to_owned(),
                usd_price: usd_price.to_owned().parse().unwrap(),
                eth_price: eth_price.to_owned().parse().unwrap(),
            };

            return Some(data);
        }
        None
    }

    async fn serialize_and_save(&self, cli: &Client, table_name: String) -> AppResult<()> {
        cli.put_item()
            .table_name(table_name)
            .item(KEY_WORK_ID, AttributeValue::S(self.work_id.to_owned()))
            .item(
                KEY_CONTRACT_ADDRESS,
                AttributeValue::S(self.contract_address.to_owned()),
            )
            .item(KEY_TOKEN_ID, AttributeValue::S(self.token_id.to_owned()))
            .item(KEY_NAME, AttributeValue::S(self.name.to_owned()))
            .item(
                KEY_DESCRIPTION,
                AttributeValue::S(self.description.to_owned()),
            )
            .item(KEY_IMAGE_URL, AttributeValue::S(self.image_url.to_owned()))
            .item(
                KEY_IMAGE_PREVIEW_URL,
                AttributeValue::S(self.image_preview_url.to_owned()),
            )
            .item(KEY_PERMALINK, AttributeValue::S(self.permalink.to_owned()))
            .item(
                KEY_USD_PRICE,
                AttributeValue::N(self.usd_price.to_owned().to_string()),
            )
            .item(
                KEY_ETH_PRICE,
                AttributeValue::N(self.eth_price.to_owned().to_string()),
            )
            .send()
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    fn primary_key(work_id: String) -> AttributeValue {
        AttributeValue::S(work_id.to_owned())
    }
}

impl Dao<asset::Asset1155> {
    pub async fn get_all(&self) -> AppResult<Vec<asset::Asset1155>> {
        let res = self
            .cli
            .scan()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .send()
            .await?;

        let mut entities: Vec<asset::Asset1155> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(asset::Asset1155::deserialize(item).unwrap())
        }

        Ok(entities)
    }

    pub async fn get(&self, work_id: String) -> AppResult<asset::Asset1155> {
        let res = self
            .cli
            .get_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_WORK_ID, asset::Asset1155::primary_key(work_id))
            .send()
            .await?;

        if res.item.is_none() {
            return Err(AppError::NotFound);
        }

        let data = res.item.unwrap();

        Ok(asset::Asset1155::deserialize(data).unwrap())
    }

    pub async fn put(&self, item: &asset::Asset1155) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: String) -> AppResult<()> {
        self.cli
            .delete_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_WORK_ID, asset::Asset1155::primary_key(id))
            .send()
            .await?;
        Ok(())
    }
}
