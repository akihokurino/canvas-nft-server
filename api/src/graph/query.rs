use crate::graph::enums::WorkStatus;
use crate::graph::outputs::user::User;
use crate::graph::outputs::work::{Work, WorkConnection, WorkEdge};
use crate::graph::outputs::OwnNft;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use app::AppError;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn me(context: &Context) -> FieldResult<User> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_publisher() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let user = context
            .user_app
            .get_me()
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(User::from(user))
    }

    async fn all_works(
        context: &Context,
        status: Option<WorkStatus>,
        next_key: Option<String>,
        limit: Option<i32>,
    ) -> FieldResult<WorkConnection> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_publisher() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let (works, next_key) = context
            .work_app
            .list(status.map_or(None, |v| Some(v.domain())), next_key, limit)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(WorkConnection {
            edges: works
                .iter()
                .map(|v| WorkEdge {
                    node: Work::from(v.to_owned()),
                })
                .collect(),
            next_key,
            total_count: None,
        })
    }

    async fn works_by_ids(context: &Context, ids: Vec<String>) -> FieldResult<Vec<Work>> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_publisher() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let works = context
            .work_app
            .get_multi(ids)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(works.iter().map(|v| Work::from(v.to_owned())).collect())
    }

    async fn work(context: &Context, id: String) -> FieldResult<Work> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_publisher() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let work = context
            .work_app
            .get(id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(Work::from(work.to_owned()))
    }

    async fn is_own_nft(context: &Context, work_id: String) -> FieldResult<OwnNft> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_publisher() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let is_own_erc721 = context
            .nft_app
            .is_own_erc721(work_id.clone())
            .await
            .map_err(FieldErrorWithCode::from)?;

        let is_own_erc1155 = context
            .nft_app
            .is_own_erc1155(work_id.clone())
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(OwnNft {
            erc721: is_own_erc721,
            erc1155: is_own_erc1155,
        })
    }
}
