use crate::ddb::Dao;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

const TABLE_NAME: &str = "canvas-nft-user";
const KEY_ID: &str = "ID";
const KEY_WALLET_ADDRESS: &str = "WalletAddress";
const KEY_WALLET_SECRET: &str = "WalletSecret";

impl user::User {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (
            Some(AttributeValue::S(id)),
            Some(AttributeValue::S(address)),
            Some(AttributeValue::S(secret)),
        ) = (
            data.get(KEY_ID),
            data.get(KEY_WALLET_ADDRESS),
            data.get(KEY_WALLET_SECRET),
        ) {
            let data = user::User {
                id: id.to_owned(),
                wallet_address: address.to_owned(),
                wallet_secret: secret.to_owned(),
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
                KEY_WALLET_ADDRESS,
                AttributeValue::S(self.wallet_address.to_owned()),
            )
            .item(
                KEY_WALLET_SECRET,
                AttributeValue::S(self.wallet_secret.to_owned()),
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

impl Dao<user::User> {
    pub async fn get(&self, id: String) -> AppResult<user::User> {
        let res = self
            .cli
            .get_item()
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .key(KEY_ID, user::User::primary_key(id))
            .send()
            .await?;

        if res.item.is_none() {
            return Err(AppError::NotFound);
        }

        let data = res.item.unwrap();

        Ok(user::User::deserialize(data).unwrap())
    }

    pub async fn get_by_wallet_address(&self, address: String) -> AppResult<user::User> {
        let res = self
            .cli
            .query()
            .index_name("WalletAddress-Index")
            .key_condition_expression("#key = :value".to_string())
            .expression_attribute_names("#key".to_string(), KEY_WALLET_ADDRESS)
            .expression_attribute_values(
                ":value".to_string(),
                AttributeValue::S(address.to_owned()),
            )
            .table_name(self.table_name_provider.with(TABLE_NAME))
            .send()
            .await?;

        let mut entities: Vec<user::User> = vec![];
        for item in res.items.unwrap_or_default() {
            entities.push(user::User::deserialize(item).unwrap())
        }

        if entities.is_empty() {
            return Err(AppError::NotFound);
        }

        Ok(entities.first().unwrap().to_owned())
    }

    pub async fn put(&self, item: &user::User) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }
}
