use crate::domain::user::User;
use crate::{AppError, AppResult};
use aws_sdk_lambda::Client;
use aws_sdk_s3::types::Blob;
use serde::{Deserialize, Serialize};

const OPEN_SEA_LAMBDA: &str =
    "arn:aws:lambda:ap-northeast-1:326914400610:function:lambda-opensea-Function-E5REgOxitk1E";

pub async fn invoke_open_sea_sdk(
    input: invoke_open_sea_sdk::Input,
) -> AppResult<invoke_open_sea_sdk::Output> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let json = serde_json::to_string(&input)?;
    println!("invoke lambda payload: {}", json);
    let resp = client
        .invoke()
        .function_name(OPEN_SEA_LAMBDA)
        .payload(Blob::new(json.into_bytes()))
        .send()
        .await?;

    let payload = resp.payload.unwrap();
    let payload = String::from_utf8(payload.into_inner()).ok().unwrap();
    let output: invoke_open_sea_sdk::Output = serde_json::from_str(&payload)?;

    if output.result != 0 {
        return Err(AppError::Internal(output.message));
    }

    Ok(output)
}

pub mod invoke_open_sea_sdk {
    use super::*;

    #[derive(Debug, Serialize)]
    pub struct Input {
        pub method: String,
        #[serde(rename(serialize = "walletAddress"))]
        pub wallet_address: String,
        #[serde(rename(serialize = "walletSecret"))]
        pub wallet_secret: String,
        #[serde(rename(serialize = "buyPayload"))]
        pub buy_payload: Option<BuyPayload>,
        #[serde(rename(serialize = "sellPayload"))]
        pub sell_payload: Option<SellPayload>,
        #[serde(rename(serialize = "transferPayload"))]
        pub transfer_payload: Option<TransferPayload>,
    }

    #[derive(Debug, Serialize)]
    pub struct BuyPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
    }

    #[derive(Debug, Serialize)]
    pub struct SellPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "schemaName"))]
        pub schema_name: String,
        #[serde(rename(serialize = "ether"))]
        pub ether: f64,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
    }

    #[derive(Debug, Serialize)]
    pub struct TransferPayload {
        #[serde(rename(serialize = "tokenAddress"))]
        pub token_address: String,
        #[serde(rename(serialize = "tokenId"))]
        pub token_id: String,
        #[serde(rename(serialize = "schemaName"))]
        pub schema_name: String,
        #[serde(rename(serialize = "transferAddress"))]
        pub transfer_address: String,
        #[serde(rename(serialize = "quantity"))]
        pub quantity: i32,
    }

    impl Input {
        pub fn sell_erc721(
            user: User,
            token_address: String,
            token_id: String,
            ether: f64,
        ) -> Self {
            Self {
                method: "sell".to_string(),
                wallet_address: user.wallet_address.to_owned(),
                wallet_secret: user.wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: Some(SellPayload {
                    token_address,
                    token_id,
                    schema_name: "ERC721".to_string(),
                    ether,
                    quantity: 1,
                }),
                transfer_payload: None,
            }
        }

        pub fn sell_erc1155(
            user: User,
            token_address: String,
            token_id: String,
            ether: f64,
        ) -> Self {
            Self {
                method: "sell".to_string(),
                wallet_address: user.wallet_address.to_owned(),
                wallet_secret: user.wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: Some(SellPayload {
                    token_address,
                    token_id,
                    schema_name: "ERC1155".to_string(),
                    ether,
                    quantity: 1,
                }),
                transfer_payload: None,
            }
        }

        pub fn transfer_erc721(
            user: User,
            token_address: String,
            token_id: String,
            to_address: String,
        ) -> Self {
            Self {
                method: "transfer".to_string(),
                wallet_address: user.wallet_address.to_owned(),
                wallet_secret: user.wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: None,
                transfer_payload: Some(TransferPayload {
                    token_address,
                    token_id,
                    schema_name: "ERC721".to_string(),
                    transfer_address: to_address,
                    quantity: 1,
                }),
            }
        }

        pub fn transfer_erc1155(
            user: User,
            token_address: String,
            token_id: String,
            to_address: String,
        ) -> Self {
            Self {
                method: "transfer".to_string(),
                wallet_address: user.wallet_address.to_owned(),
                wallet_secret: user.wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: None,
                transfer_payload: Some(TransferPayload {
                    token_address,
                    token_id,
                    schema_name: "ERC1155".to_string(),
                    transfer_address: to_address,
                    quantity: 1,
                }),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Output {
        pub message: String,
        pub result: i32,
    }
}
