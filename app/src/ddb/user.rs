use crate::ddb::Dao;
use crate::domain::*;
use crate::{AppError, AppResult};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;

const TABLE_NAME: &str = "canvas-nft-user";
const KEY_ID: &str = "ID";
const KEY_ADDRESS_PATH: &str = "Address";

impl user::User {
    fn deserialize(data: HashMap<String, AttributeValue>) -> Option<Self> {
        if let (Some(AttributeValue::S(id)), Some(AttributeValue::S(address))) =
            (data.get(KEY_ID), data.get(KEY_ADDRESS_PATH))
        {
            let data = user::User {
                id: id.to_owned(),
                address: address.to_owned(),
            };

            return Some(data);
        }
        None
    }

    async fn serialize_and_save(&self, cli: &Client, table_name: String) -> AppResult<()> {
        cli.put_item()
            .table_name(table_name)
            .item(KEY_ID, AttributeValue::S(self.id.to_owned()))
            .item(KEY_ADDRESS_PATH, AttributeValue::S(self.address.to_owned()))
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

    pub async fn put(&self, item: &user::User) -> AppResult<()> {
        item.serialize_and_save(&self.cli, self.table_name_provider.with(TABLE_NAME))
            .await?;
        Ok(())
    }
}
