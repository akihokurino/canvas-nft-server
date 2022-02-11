use crate::ddb::Dao;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::model::*;
use aws_sdk_dynamodb::Client;
use std::collections::{HashMap, HashSet};

const TABLE_NAME: &str = "canvas-store-nft";
const KEY_WORK_ID: &str = "WorkID";
const KEY_ADDRESS: &str = "Address";
const KEY_TOKEN_ID: &str = "TokenID";
const KEY_NAME: &str = "Name";
const KEY_DESCRIPTION: &str = "Description";
const KEY_IMAGE_URL: &str = "ImageURL";
const KEY_IMAGE_PREVIEW_URL: &str = "ImagePreviewURL";
const KEY_PERMALINK: &str = "Permalink";
const KEY_USD_PRICE: &str = "UsdPrice";
const KEY_ETH_PRICE: &str = "EthPrice";

impl nft::NFT {
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
            data.get(KEY_ADDRESS),
            data.get(KEY_TOKEN_ID),
            data.get(KEY_NAME),
            data.get(KEY_DESCRIPTION),
            data.get(KEY_IMAGE_URL),
            data.get(KEY_IMAGE_PREVIEW_URL),
            data.get(KEY_PERMALINK),
            data.get(KEY_USD_PRICE),
            data.get(KEY_ETH_PRICE),
        ) {
            let data = nft::NFT {
                work_id: work_id.to_owned(),
                address: address.to_owned(),
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
            .item(KEY_ADDRESS, AttributeValue::S(self.address.to_owned()))
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

impl Dao<nft::NFT> {
    pub async fn get_multi(&self, work_ids: Vec<String>) -> AppResult<Vec<nft::NFT>> {
        if work_ids.is_empty() {
            return Ok(vec![]);
        }
        let ids: HashSet<String> = work_ids.into_iter().collect();
        let mut builder = keys_and_attributes::Builder::default();
        for id in ids {
            let mut key: HashMap<String, AttributeValue> = HashMap::new();
            key.insert(KEY_WORK_ID.to_string(), AttributeValue::S(id.to_owned()));
            builder = builder.keys(key);
        }

        let res = self
            .cli
            .batch_get_item()
            .request_items(self.table_name_provider.with(TABLE_NAME), builder.build())
            .send()
            .await?;

        let mut entities: Vec<nft::NFT> = vec![];
        for (table, data) in res.responses.unwrap_or_default() {
            if table == self.table_name_provider.with(TABLE_NAME) {
                for item in data {
                    entities.push(nft::NFT::deserialize(item).unwrap())
                }
            }
        }

        Ok(entities)
    }

    pub async fn get(&self, work_id: String) -> AppResult<nft::NFT> {
        let res = self
            .cli
            .get_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_WORK_ID, nft::NFT::primary_key(work_id))
            .send()
            .await?;

        if res.item.is_none() {
            return Err(AppError::NotFound);
        }

        let data = res.item.unwrap();

        Ok(nft::NFT::deserialize(data).unwrap())
    }

    pub async fn put(&self, item: &nft::NFT) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }
}
