pub mod application;
pub mod aws;
mod csv_loader;
pub mod dataloader;
mod ddb;
pub mod domain;
mod ethereum;
pub mod internal_api;
mod open_sea;

use aws_sdk_cognitoidentityprovider::error::{
    AdminCreateUserError, AdminGetUserError, AdminSetUserPasswordError,
};
use aws_sdk_dynamodb::error::{
    BatchGetItemError, DeleteItemError, GetItemError, GetItemErrorKind, PutItemError, QueryError,
    ScanError,
};
use aws_sdk_dynamodb::types::SdkError;
use aws_sdk_s3::error::{GetObjectError, GetObjectErrorKind, PutObjectError};
use aws_sdk_sesv2::error::SendEmailError;
use aws_sdk_sns::error::PublishError;
use http::StatusCode;
use thiserror::Error as ThisErr;

pub const WORK_CSV_PATH_PREFIX: &str = "work_csv";
pub const THUMBNAIL_CSV_PATH_PREFIX: &str = "thumbnail_csv";
pub const NFT_721_ASSET_PATH_PREFIX: &str = "721_asset";
pub const NFT_1155_ASSET_PATH_PREFIX: &str = "1155_asset";

#[derive(ThisErr, Debug, PartialOrd, PartialEq, Clone)]
pub enum AppError {
    #[error("不正なパラメーターです: {0}")]
    BadRequest(String),
    #[error("認証エラーです")]
    UnAuthenticate,
    #[error("禁止された行為です")]
    Forbidden,
    #[error("指定されたリソースが見つかりません")]
    NotFound,
    #[error("サーバーエラーです: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl From<SdkError<PutItemError>> for AppError {
    fn from(e: SdkError<PutItemError>) -> Self {
        let msg = format!("database write error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<ScanError>> for AppError {
    fn from(e: SdkError<ScanError>) -> Self {
        let msg = format!("database scan error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<QueryError>> for AppError {
    fn from(e: SdkError<QueryError>) -> Self {
        let msg = format!("database query error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<GetItemError>> for AppError {
    fn from(e: SdkError<GetItemError>) -> Self {
        match &e {
            SdkError::ServiceError { err, raw: _ } => match err.kind {
                GetItemErrorKind::ResourceNotFoundException(_) => Self::NotFound,
                _ => {
                    let msg = format!("database get error: {:?}", e);
                    Self::Internal(msg)
                }
            },
            _ => {
                let msg = format!("database get error: {:?}", e);
                Self::Internal(msg)
            }
        }
    }
}

impl From<SdkError<BatchGetItemError>> for AppError {
    fn from(e: SdkError<BatchGetItemError>) -> Self {
        let msg = format!("database batch get error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<DeleteItemError>> for AppError {
    fn from(e: SdkError<DeleteItemError>) -> Self {
        let msg = format!("database delete error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<GetObjectError>> for AppError {
    fn from(e: SdkError<GetObjectError>) -> Self {
        match &e {
            SdkError::ServiceError { err, raw: _ } => match err.kind {
                GetObjectErrorKind::NoSuchKey(_) => Self::NotFound,
                _ => {
                    let msg = format!("s3 get error: {:?}", e);
                    Self::Internal(msg)
                }
            },
            _ => {
                let msg = format!("s3 get error: {:?}", e);
                Self::Internal(msg)
            }
        }
    }
}

impl From<SdkError<PutObjectError>> for AppError {
    fn from(e: SdkError<PutObjectError>) -> Self {
        let msg = format!("s3 write error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<PublishError>> for AppError {
    fn from(e: SdkError<PublishError>) -> Self {
        let msg = format!("sns publish error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<SendEmailError>> for AppError {
    fn from(e: SdkError<SendEmailError>) -> Self {
        let msg = format!("ses send error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<AdminGetUserError>> for AppError {
    fn from(e: SdkError<AdminGetUserError>) -> Self {
        let msg = format!("cognito get user error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<AdminCreateUserError>> for AppError {
    fn from(e: SdkError<AdminCreateUserError>) -> Self {
        let msg = format!("cognito create user error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<SdkError<AdminSetUserPasswordError>> for AppError {
    fn from(e: SdkError<AdminSetUserPasswordError>) -> Self {
        let msg = format!("cognito set user password error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<jsonwebtokens_cognito::Error> for AppError {
    fn from(e: jsonwebtokens_cognito::Error) -> Self {
        match e {
            jsonwebtokens_cognito::Error::NetworkError(_) => {
                let msg = "cognito network error".to_string();
                Self::Internal(msg)
            }
            jsonwebtokens_cognito::Error::CacheMiss(_) => {
                let msg = "cognito internal error".to_string();
                Self::Internal(msg)
            }
            _ => Self::UnAuthenticate,
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        let code = e.status().unwrap_or_default();
        if code == StatusCode::from_u16(400).unwrap() {
            return Self::BadRequest(e.to_string());
        }
        if code == StatusCode::from_u16(401).unwrap() {
            return Self::UnAuthenticate;
        }
        if code == StatusCode::from_u16(403).unwrap() {
            return Self::Forbidden;
        }
        if code == StatusCode::from_u16(404).unwrap() {
            return Self::NotFound;
        }

        let msg = format!("http error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        let msg = format!("json parse error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<base64::DecodeError> for AppError {
    fn from(e: base64::DecodeError) -> Self {
        let msg = format!("base64 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<csv::Error> for AppError {
    fn from(e: csv::Error) -> Self {
        let msg = format!("csv error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        let msg = format!("jwt token error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::Error> for AppError {
    fn from(e: web3::Error) -> Self {
        let msg = format!("web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::ethabi::Error> for AppError {
    fn from(e: web3::ethabi::Error) -> Self {
        let msg = format!("web3 error: {:?}", e);
        Self::Internal(msg)
    }
}

impl From<web3::contract::Error> for AppError {
    fn from(e: web3::contract::Error) -> Self {
        let msg = format!("web3 error: {:?}", e);
        Self::Internal(msg)
    }
}
