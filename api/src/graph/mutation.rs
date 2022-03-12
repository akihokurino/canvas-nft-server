use crate::graph::inputs::{
    BindNftToWorkInput, CreateNft1155Input, CreateNftInput, CreateThumbnailInput, CreateWorkInput,
    RegisterUserInput, UpdateWorkStatusInput,
};
use crate::graph::outputs::user::User;
use crate::graph::outputs::PreSignUploadUrl;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use app::AppError;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn register_user(context: &Context, input: RegisterUserInput) -> FieldResult<User> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_service() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let user = context
            .service_user_app
            .register(input.address)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(User::from(user))
    }

    async fn create_work(context: &Context, input: CreateWorkInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let payload = app::aws::sns::CreateWorkPayload {
            executor_id: auth_user.user_id().unwrap(),
            prefix: String::from(app::WORK_CSV_PATH_PREFIX),
            file_name: input.file_name,
        };

        app::aws::sns::publish(app::aws::sns::Task::CreateWork(payload))
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn create_thumbnail(context: &Context, input: CreateThumbnailInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let payload = app::aws::sns::CreateThumbnailPayload {
            executor_id: auth_user.user_id().unwrap(),
            prefix: String::from(app::THUMBNAIL_CSV_PATH_PREFIX),
            file_name: input.file_name,
        };

        app::aws::sns::publish(app::aws::sns::Task::CreateThumbnail(payload))
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn pre_sign_upload_work(context: &Context) -> FieldResult<PreSignUploadUrl> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let (url, file_name) = context
            .admin_work_app
            .pre_sign_for_upload(String::from(app::WORK_CSV_PATH_PREFIX))
            .await?;

        Ok(PreSignUploadUrl {
            url: url.to_string(),
            file_name,
        })
    }

    async fn pre_sign_upload_thumbnail(context: &Context) -> FieldResult<PreSignUploadUrl> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let (url, file_name) = context
            .admin_work_app
            .pre_sign_for_upload(String::from(app::THUMBNAIL_CSV_PATH_PREFIX))
            .await?;

        Ok(PreSignUploadUrl {
            url: url.to_string(),
            file_name,
        })
    }

    async fn update_work_status(
        context: &Context,
        input: UpdateWorkStatusInput,
    ) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_work_app
            .update_status(input.id, input.status.domain())
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn create_nft(context: &Context, input: CreateNftInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .create_erc721(input.work_id, input.thumbnail_url, input.point, input.level)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn create_nft_1155(context: &Context, input: CreateNft1155Input) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .create_erc1155(
                input.work_id,
                input.thumbnail_url,
                input.point,
                input.level,
                input.amount as u32,
            )
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn bind_nft_to_work(context: &Context, input: BindNftToWorkInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .bind_work(input.work_id, input.contract_address, input.token_id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn delete_work(context: &Context, id: String) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_work_app
            .delete(id)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }
}
