use crate::domain::user::User;
use crate::{AppError, AppResult};
use aws_sdk_lambda::Client;
use aws_sdk_s3::types::Blob;
use serde::{Deserialize, Serialize};

const OPEN_SEA_LAMBDA: &str =
    "arn:aws:lambda:ap-northeast-1:326914400610:function:lambda-opensea-Function-tddkoUUKqXu9";

pub async fn invoke_open_sea_sdk(input: Input) -> AppResult<Output> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let json = serde_json::to_string(&input)?;

    let resp = client
        .invoke()
        .function_name(OPEN_SEA_LAMBDA)
        .payload(Blob::new(json.into_bytes()))
        .send()
        .await?;

    let payload = resp.payload.unwrap();
    let payload = String::from_utf8(payload.into_inner()).ok().unwrap();
    let output: Output = serde_json::from_str(&payload)?;

    if output.result != 0 {
        return Err(AppError::Internal("lambda opensea error".to_string()));
    }

    Ok(output)
}

#[derive(Debug, Serialize)]
pub struct Input {
    pub task: String,
    pub wallet_address: String,
    pub wallet_secret: String,
    pub token_address: String,
    pub token_id: String,
    pub sell_ether: f64,
    pub schema_name: String,
    pub transfer_address: String,
    pub transfer_amount: i32,
}

impl Input {
    fn sell(user: User, token_address: String, token_id: String, ether: f64) -> Self {
        Self {
            task: "sell".to_string(),
            wallet_address: user.wallet_address.to_owned(),
            wallet_secret: user.wallet_secret.to_owned(),
            token_address,
            token_id,
            sell_ether: ether,
            schema_name: "".to_string(),
            transfer_address: "".to_string(),
            transfer_amount: 0,
        }
    }

    fn transfer_721(
        user: User,
        token_address: String,
        token_id: String,
        to_address: String,
    ) -> Self {
        Self {
            task: "transfer".to_string(),
            wallet_address: user.wallet_address.to_owned(),
            wallet_secret: user.wallet_secret.to_owned(),
            token_address,
            token_id,
            sell_ether: 0.0,
            schema_name: "ERC721".to_string(),
            transfer_address: to_address,
            transfer_amount: 1,
        }
    }

    fn transfer_1155(
        user: User,
        token_address: String,
        token_id: String,
        to_address: String,
    ) -> Self {
        Self {
            task: "transfer".to_string(),
            wallet_address: user.wallet_address.to_owned(),
            wallet_secret: user.wallet_secret.to_owned(),
            token_address,
            token_id,
            sell_ether: 0.0,
            schema_name: "ERC1155".to_string(),
            transfer_address: to_address,
            transfer_amount: 1,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Output {
    pub message: String,
    pub result: i32,
}
