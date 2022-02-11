use crate::open_sea::{CallInput, Client};
use crate::{AppError, AppResult};
use reqwest::Method;
use serde::{Deserialize, Serialize};

impl Client {
    pub async fn get_asset(&self, input: get_asset::Input) -> AppResult<get_asset::Output> {
        #[derive(Debug, Serialize)]
        struct Body {}

        let body = Body {};

        println!("json body: {}", serde_json::to_string(&body).unwrap());

        let query = vec![];

        self.call(CallInput {
            method: Method::GET,
            path: format!("/api/v1/asset/{}/{}", input.address, input.token_id).to_string(),
            body: Some(
                serde_json::to_string(&body)
                    .map_err(|e| AppError::Internal(e.to_string()))?
                    .into(),
            ),
            query,
        })
        .await?
        .error_for_status()?
        .json::<get_asset::Output>()
        .await
        .map_err(AppError::from)
    }
}

pub mod get_asset {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Input {
        pub address: String,
        pub token_id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Output {
        pub id: i32,
        pub image_url: String,
        pub image_preview_url: String,
        pub name: String,
        pub description: String,
        pub permalink: String,
        pub collection: Collection,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Collection {
        pub payment_tokens: Vec<PaymentToken>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PaymentToken {
        pub symbol: Option<String>,
        pub eth_price: Option<f64>,
        pub usd_price: Option<f64>,
    }
}
