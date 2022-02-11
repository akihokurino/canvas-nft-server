mod enums;
mod inputs;
mod mutation;
mod outputs;
mod query;

use crate::graph::mutation::MutationRoot;
use crate::graph::query::QueryRoot;
use app::application;
use app::dataloader;
use app::internal_api;
use app::AppError;
use convert_case::{Case, Casing};
use juniper::{EmptySubscription, FieldError, RootNode};
use strum_macros::Display as StrumDisplay;

#[derive(Clone, PartialEq)]
pub enum AuthUser {
    // 管理ユーザー
    Admin(String),
    // 一般ユーザー（ログイン済）
    Service(String),
    // 一般ユーザー（未ログイン）
    None,
}

impl AuthUser {
    pub fn user_id(&self) -> Option<String> {
        match self {
            AuthUser::Admin(id) => Some(id.to_owned()),
            AuthUser::Service(id) => Some(id.to_owned()),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            AuthUser::Admin(_id) => true,
            _ => false,
        }
    }
}

pub struct Context {
    pub auth_user: AuthUser,
    pub admin_work_app: application::admin::work::Application,
    pub admin_nft_app: application::admin::nft::Application,
    pub service_work_app: application::service::work::Application,
    pub thumbnail_by_work_loader: dataloader::thumbnail_by_work::Loader,
    pub nft_by_work_loader: dataloader::nft_by_work::Loader,
    pub internal_api: internal_api::Client,
}

impl juniper::Context for Context {}

impl Context {
    pub async fn new(auth_user: AuthUser) -> Self {
        let admin_work_app = application::admin::work::Application::new(
            auth_user.user_id().clone().unwrap_or_default(),
        )
        .await;
        let admin_nft_app = application::admin::nft::Application::new(
            auth_user.user_id().clone().unwrap_or_default(),
        )
        .await;
        let service_work_app = application::service::work::Application::new(
            auth_user.user_id().clone().unwrap_or_default(),
        )
        .await;

        let thumbnail_by_work_loader: dataloader::thumbnail_by_work::Loader =
            dataloader::thumbnail_by_work::Batcher::new_loader();
        let nft_by_work_loader: dataloader::nft_by_work::Loader =
            dataloader::nft_by_work::Batcher::new_loader();

        let internal_api = internal_api::Client::new();

        Self {
            auth_user,
            admin_work_app,
            admin_nft_app,
            service_work_app,
            thumbnail_by_work_loader,
            nft_by_work_loader,
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
