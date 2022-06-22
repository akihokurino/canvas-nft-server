mod enums;
mod inputs;
mod mutation;
mod outputs;
mod query;

use crate::graph::mutation::MutationRoot;
use crate::graph::query::QueryRoot;
use app::application;
use app::dataloader;
use app::domain::user::AuthUser;
use app::internal_api;
use app::AppError;
use convert_case::{Case, Casing};
use juniper::{EmptySubscription, FieldError, RootNode};
use strum_macros::Display as StrumDisplay;

pub struct Context {
    pub auth_user: AuthUser,
    pub work_app: application::work::Application,
    pub nft_app: application::nft::Application,
    pub user_app: application::user::Application,
    pub thumbnail_by_work_loader: dataloader::thumbnail_by_work::Loader,
    pub asset721_by_work_loader: dataloader::asset721_by_work::Loader,
    pub asset1155_by_work_loader: dataloader::asset1155_by_work::Loader,
    pub internal_api: internal_api::Client,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn new(auth_user: AuthUser) -> Self {
        let work_app =
            application::work::Application::new(auth_user.user_id().clone().unwrap_or_default())
                .await;
        let nft_app =
            application::nft::Application::new(auth_user.user_id().clone().unwrap_or_default())
                .await;
        let user_app =
            application::user::Application::new(auth_user.user_id().clone().unwrap_or_default())
                .await;

        let thumbnail_by_work_loader: dataloader::thumbnail_by_work::Loader =
            dataloader::thumbnail_by_work::Batcher::new_loader();
        let asset721_by_work_loader: dataloader::asset721_by_work::Loader =
            dataloader::asset721_by_work::Batcher::new_loader();
        let asset1155_by_work_loader: dataloader::asset1155_by_work::Loader =
            dataloader::asset1155_by_work::Batcher::new_loader();

        let internal_api = internal_api::Client::new();

        Self {
            auth_user,
            work_app,
            nft_app,
            user_app,
            thumbnail_by_work_loader,
            asset721_by_work_loader,
            asset1155_by_work_loader,
            internal_api,
        }
    }
}

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

#[derive(StrumDisplay, Debug)]
pub enum FieldErrorCode {
    BadRequest,
    UnAuthenticate,
    NotFound,
    Forbidden,
    Internal,
}

pub struct FieldErrorWithCode {
    err: AppError,
    code: FieldErrorCode,
}

impl From<AppError> for FieldErrorWithCode {
    fn from(err: AppError) -> Self {
        FieldErrorWithCode {
            err: err.clone(),
            code: match err {
                AppError::BadRequest(_) => FieldErrorCode::BadRequest,
                AppError::UnAuthenticate => FieldErrorCode::UnAuthenticate,
                AppError::Forbidden => FieldErrorCode::Forbidden,
                AppError::NotFound => FieldErrorCode::NotFound,
                AppError::Internal(_) => FieldErrorCode::Internal,
            },
        }
    }
}

impl From<FieldErrorWithCode> for FieldError {
    fn from(v: FieldErrorWithCode) -> Self {
        let code = v.code.to_string().to_case(Case::UpperSnake);

        FieldError::new(
            v.err,
            graphql_value!({
                "code": code,
            }),
        )
    }
}
