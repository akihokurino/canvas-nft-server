use crate::AppResult;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

mod asset;
mod thumbnail;
mod user;
mod work;

pub struct Dao<T> {
    cli: Client,
    table_name_provider: TableNameProvider,
    _phantom: PhantomData<fn() -> T>,
}

impl<T> Dao<T> {
    pub async fn new() -> Self {
        let shared_config = aws_config::load_from_env().await;
        let client = Client::new(&shared_config);
        let table_name_provider = TableNameProvider::new();

        Self {
            cli: client,
            table_name_provider,
            _phantom: PhantomData,
        }
    }
}

pub struct TableNameProvider {
    pub prefix: String,
}

impl TableNameProvider {
    pub fn new() -> Self {
        Self {
            prefix: "".to_string(),
        }
    }

    pub fn with(&self, base_name: &str) -> String {
        format!("{}{}", self.prefix.as_str(), base_name)
    }
}

pub struct PagingKey {
    pub val: Option<HashMap<String, AttributeValue>>,
}

impl PagingKey {
    pub fn from(val: Option<HashMap<String, AttributeValue>>) -> Self {
        Self { val }
    }

    pub fn decode(from: Option<String>) -> AppResult<Self> {
        if from.is_none() {
            return Ok(Self { val: None });
        }

        let bytes = base64::decode::<String>(from.unwrap())?;
        let json = bytes.iter().map(|&s| s as char).collect::<String>();
        let tmp = serde_json::from_str::<HashMap<String, EncodableAttributeValue>>(json.as_str())?;
        let mut val: HashMap<String, AttributeValue> = HashMap::new();
        for (k, v) in tmp {
            match v {
                EncodableAttributeValue::S(rv) => {
                    val.insert(k.to_owned(), AttributeValue::S(rv));
                }
                EncodableAttributeValue::N(rv) => {
                    val.insert(k.to_owned(), AttributeValue::N(rv));
                }
            }
        }
        Ok(Self { val: Some(val) })
    }

    pub fn encode(&self) -> Option<String> {
        if self.val.is_none() {
            return None;
        }

        let mut tmp: HashMap<String, EncodableAttributeValue> = HashMap::new();
        for (k, v) in self.val.to_owned().unwrap() {
            if v.is_s() {
                tmp.insert(
                    k.to_owned(),
                    EncodableAttributeValue::S(v.as_s().unwrap().to_owned()),
                );
            }
            if v.is_n() {
                tmp.insert(
                    k.to_owned(),
                    EncodableAttributeValue::N(v.as_n().unwrap().to_owned()),
                );
            }
        }
        let json = serde_json::to_string(&tmp).unwrap();
        Some(base64::encode::<String>(json))
    }
}

#[derive(Serialize, Deserialize)]
pub enum EncodableAttributeValue {
    S(String),
    N(String),
}
