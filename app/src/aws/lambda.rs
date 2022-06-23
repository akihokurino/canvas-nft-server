use crate::domain::user::User;
use crate::{AppError, AppResult};
use aws_sdk_lambda::Client;
use aws_sdk_s3::types::Blob;
use serde::{Deserialize, Serialize};
use std::env;

pub async fn invoke_open_sea_sdk(
    input: invoke_open_sea_sdk::Input,
) -> AppResult<invoke_open_sea_sdk::Output> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let arn = env::var("LAMBDA_OPENSEA_ARN").expect("should set lambda opensea arn");

    let json = serde_json::to_string(&input)?;
    let resp = client
        .invoke()
        .function_name(arn)
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
        #[serde(rename(serialize = "createMetadataPayload"))]
        pub create_metadata_payload: Option<CreateMetadataPayload>,
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

    #[derive(Debug, Serialize)]
    pub struct CreateMetadataPayload {
        #[serde(rename(serialize = "name"))]
        pub name: String,
        #[serde(rename(serialize = "description"))]
        pub description: String,
        #[serde(rename(serialize = "externalUrl"))]
        pub external_url: String,
        #[serde(rename(serialize = "imageBase64"))]
        pub image_base64: String,
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
                create_metadata_payload: None,
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
                create_metadata_payload: None,
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
                create_metadata_payload: None,
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
                create_metadata_payload: None,
            }
        }

        pub fn create_metadata(
            user: User,
            name: String,
            description: String,
            external_url: String,
            image_base64: String,
        ) -> Self {
            Self {
                method: "createMetadata".to_string(),
                wallet_address: user.wallet_address.to_owned(),
                wallet_secret: user.wallet_secret.to_owned(),
                buy_payload: None,
                sell_payload: None,
                transfer_payload: None,
                create_metadata_payload: Some(CreateMetadataPayload {
                    name,
                    description,
                    external_url,
                    image_base64,
                }),
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Output {
        #[serde(rename(deserialize = "message"))]
        pub message: String,
        #[serde(rename(deserialize = "result"))]
        pub result: i32,
        #[serde(rename(deserialize = "ipfsResponse"))]
        pub ipfs_response: Option<OutputIPFS>,
    }

    #[derive(Debug, Deserialize)]
    pub struct OutputIPFS {
        #[serde(rename(deserialize = "hash"))]
        pub hash: String,
        #[serde(rename(deserialize = "url"))]
        pub url: String,
    }
}
