use crate::graph::enums::WorkStatus;
use crate::graph::outputs::user::User;
use crate::graph::outputs::work::{Work, WorkConnection, WorkEdge};
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use crate::AuthUser;
use app::AppError;
use juniper::FieldResult;

pub struct QueryRoot;

#[juniper::graphql_object(Context = Context)]
impl QueryRoot {
    async fn me(context: &Context) -> FieldResult<User> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let user = context
            .admin_user_app
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
        let (works, next_key) = match context.auth_user.to_owned() {
            AuthUser::Admin(_) => context
                .admin_work_app
                .list(status.map_or(None, |v| Some(v.domain())), next_key, limit)
                .await
                .map_err(FieldErrorWithCode::from)?,
            _ => (vec![], None),
        };

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
        let works = match context.auth_user.to_owned() {
            AuthUser::Admin(_) => context
                .admin_work_app
                .get_multi(ids)
                .await
                .map_err(FieldErrorWithCode::from)?,
            _ => vec![],
        };

        Ok(works.iter().map(|v| Work::from(v.to_owned())).collect())
    }

    async fn work(context: &Context, id: String) -> FieldResult<Work> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let work = context
            .admin_work_app
            .get(id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(Work::from(work.to_owned()))
    }

    async fn owner_of_nft721(context: &Context, work_id: String) -> FieldResult<String> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let result = context
            .ethereum_cli
            .get_erc721_owner_of(work_id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(result)
    }

    async fn is_own_nft721(
        context: &Context,
        address: String,
        work_id: String,
    ) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let result = context
            .ethereum_cli
            .is_erc721_owned(address, work_id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(result)
    }
}
