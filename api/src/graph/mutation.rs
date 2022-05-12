use crate::graph::inputs::{
    CreateNft1155Input, CreateNft721Input, ImportThumbnailInput, ImportWorkInput,
    RegisterUserInput, SellNftInput,
};
use crate::graph::outputs::PreSignUploadUrl;
use crate::graph::Context;
use crate::graph::FieldErrorWithCode;
use app::AppError;
use juniper::FieldResult;

pub struct MutationRoot;

#[juniper::graphql_object(Context = Context)]
impl MutationRoot {
    async fn register_user(context: &Context, input: RegisterUserInput) -> FieldResult<String> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_master() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        let user_id = context
            .admin_user_app
            .register(
                input.email,
                input.password,
                input.wallet_address,
                input.wallet_secret,
            )
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(user_id.to_owned())
    }

    async fn import_work(context: &Context, input: ImportWorkInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_master() {
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

    async fn import_thumbnail(context: &Context, input: ImportThumbnailInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_master() {
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
        if !auth_user.is_master() {
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
        if !auth_user.is_master() {
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

    async fn create_nft_721(context: &Context, input: CreateNft721Input) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .prepare_erc721(input.work_id, input.gs_path, input.point, input.level)
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
            .prepare_erc1155(
                input.work_id,
                input.gs_path,
                input.point,
                input.level,
                input.amount as u32,
            )
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

    async fn sell_nft_721(context: &Context, input: SellNftInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .sell_721(input.work_id, input.ether)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }

    async fn sell_nft_1155(context: &Context, input: SellNftInput) -> FieldResult<bool> {
        let auth_user = context.auth_user.to_owned();
        if !auth_user.is_admin() {
            return Err(FieldErrorWithCode::from(AppError::UnAuthenticate).into());
        }

        context
            .admin_nft_app
            .sell_1155(input.work_id, input.ether)
            .await
            .map_err(FieldErrorWithCode::from)?;

        Ok(true)
    }
}
