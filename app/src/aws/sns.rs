use crate::{AppError, AppResult};
use aws_sdk_sns::Client;
use serde::{Deserialize, Serialize};

pub enum Task {
    CreateWork(CreateWorkPayload),
    CreateThumbnail(CreateThumbnailPayload),
    MintNFT721(MintNft721Payload),
    MintNFT1155(MintNft1155Payload),
}

impl Task {
    pub fn subject_number(&self) -> i32 {
        match self {
            Task::CreateWork(_) => 1,
            Task::CreateThumbnail(_) => 2,
            Task::MintNFT721(_) => 3,
            Task::MintNFT1155(_) => 4,
        }
    }

    pub fn message(&self) -> AppResult<String> {
        match self {
            Task::CreateWork(data) => serde_json::to_string(data).map_err(AppError::from),
            Task::CreateThumbnail(data) => serde_json::to_string(data).map_err(AppError::from),
            Task::MintNFT721(data) => serde_json::to_string(data).map_err(AppError::from),
            Task::MintNFT1155(data) => serde_json::to_string(data).map_err(AppError::from),
        }
    }

    pub fn from_sns(raw_number: i32, raw_message: String) -> AppResult<Self> {
        if raw_number == 1 {
            let payload: CreateWorkPayload = serde_json::from_str(raw_message.as_str())?;
            Ok(Task::CreateWork(payload))
        } else if raw_number == 2 {
            let payload: CreateThumbnailPayload = serde_json::from_str(raw_message.as_str())?;
            Ok(Task::CreateThumbnail(payload))
        } else if raw_number == 3 {
            let payload: MintNft721Payload = serde_json::from_str(raw_message.as_str())?;
            Ok(Task::MintNFT721(payload))
        } else if raw_number == 4 {
            let payload: MintNft1155Payload = serde_json::from_str(raw_message.as_str())?;
            Ok(Task::MintNFT1155(payload))
        } else {
            Err(AppError::BadRequest("不明なタスクです".to_string()))
        }
    }

    pub fn topic_arn(&self) -> String {
        "arn:aws:sns:ap-northeast-1:326914400610:canvas-nft-topic".to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateWorkPayload {
    pub executor_id: String,
    pub prefix: String,
    pub file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateThumbnailPayload {
    pub executor_id: String,
    pub prefix: String,
    pub file_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MintNft721Payload {
    pub executor_id: String,
    pub work_id: String,
    pub ipfs_hash: String,
    pub s3_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct MintNft1155Payload {
    pub executor_id: String,
    pub work_id: String,
    pub amount: u32,
    pub ipfs_hash: String,
    pub s3_key: String,
}

pub async fn publish(task: Task) -> AppResult<()> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let message = task.message()?;

    client
        .publish()
        .topic_arn(task.topic_arn().as_str())
        .subject(task.subject_number().to_string())
        .message(message)
        .send()
        .await?;

    Ok(())
}
