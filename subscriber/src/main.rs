use app::aws::*;
use app::{application, AppError, AppResult};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    ssm::load_env().await;
    lambda_runtime::run(handler_fn(exec)).await?;
    Ok(())
}

async fn exec(event: Value, _: Context) -> Result<(), Error> {
    let data = get_message_from_sns_event(event)
        .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
    let task = sns::Task::from_sns(data.0, data.1)
        .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;

    match task {
        sns::Task::CreateWork(data) => {
            println!(
                "exec create work, executor_id: {}, prefix: {}, file_name: {}",
                data.executor_id, data.prefix, data.file_name
            );

            let email = cognito::get_email(data.executor_id.clone()).await?;

            let admin_work_app =
                application::work::Application::new(data.executor_id.clone()).await;
            let res = admin_work_app
                .import_work(data.prefix, data.file_name)
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)));

            if let Err(e) = res {
                ses::send(
                    email,
                    "Workインポート処理に失敗しました".to_string(),
                    format!("失敗しました。\n{:?}", e).to_string(),
                )
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
                return Err(e.into());
            }

            ses::send(
                email,
                "Workインポート処理に成功しました".to_string(),
                "成功しました。".to_string(),
            )
            .await
            .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;

            println!("success create work");
        }
        sns::Task::CreateThumbnail(data) => {
            println!(
                "exec create thumbnail, executor_id: {}, prefix: {}, file_name: {}",
                data.executor_id, data.prefix, data.file_name
            );

            let email = cognito::get_email(data.executor_id.clone()).await?;

            let admin_work_app =
                application::work::Application::new(data.executor_id.clone()).await;
            let res = admin_work_app
                .import_thumbnail(data.prefix, data.file_name)
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)));

            if let Err(e) = res {
                ses::send(
                    email,
                    "Thumbnailインポート処理に失敗しました".to_string(),
                    format!("失敗しました。\n{:?}", e).to_string(),
                )
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
                return Err(e.into());
            }

            ses::send(
                email,
                "Thumbnailインポート処理に成功しました".to_string(),
                "成功しました。".to_string(),
            )
            .await
            .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;

            println!("success create thumbnail");
        }
        sns::Task::MintNFT721(data) => {
            println!(
                "mint 721 token, executor_id: {}, work_id: {}",
                data.executor_id,
                data.work_id.clone()
            );

            let email = cognito::get_email(data.executor_id.clone()).await?;

            let admin_nft_app = application::nft::Application::new(data.executor_id.clone()).await;
            let res = admin_nft_app
                .mint_erc721(data.work_id.clone())
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)));

            if let Err(e) = res {
                ses::send(
                    email,
                    "ERC721の発行に失敗しました".to_string(),
                    format!("失敗しました。\n{:?}", e).to_string(),
                )
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
                return Err(e.into());
            }

            ses::send(
                email,
                "ERC721の発行に成功しました".to_string(),
                "成功しました。".to_string(),
            )
            .await
            .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;

            println!("success mint 721 token");
        }
        sns::Task::MintNFT1155(data) => {
            println!(
                "mint 1155 token, executor_id: {}, work_id: {}",
                data.executor_id,
                data.work_id.clone()
            );

            let email = cognito::get_email(data.executor_id.clone()).await?;

            let admin_nft_app = application::nft::Application::new(data.executor_id.clone()).await;
            let res = admin_nft_app
                .mint_erc1155(data.work_id.clone(), data.amount.clone())
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)));

            if let Err(e) = res {
                ses::send(
                    email,
                    "ERC1155の発行に失敗しました".to_string(),
                    format!("失敗しました。\n{:?}", e).to_string(),
                )
                .await
                .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
                return Err(e.into());
            }

            ses::send(
                email,
                "ERC1155の発行に成功しました".to_string(),
                "成功しました。".to_string(),
            )
            .await
            .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;

            println!("success mint 721 token");
        }
    };

    Ok(())
}

fn get_message_from_sns_event(event: Value) -> AppResult<(i32, String)> {
    match event {
        Value::Object(data) => {
            let data = data.get("Records");
            if data.is_none() {
                return Err(AppError::BadRequest(format!("record nil: {:?}", data)));
            }
            match data.unwrap() {
                Value::Array(data) => {
                    if data.first().is_none() {
                        return Err(AppError::BadRequest(format!("record empty: {:?}", data)));
                    }

                    let target = data.first().unwrap();
                    match target {
                        Value::Object(data) => {
                            let data = data.get("Sns");
                            if data.is_none() {
                                return Err(AppError::BadRequest(format!("sns nil: {:?}", data)));
                            }
                            match data.unwrap() {
                                Value::Object(data) => {
                                    let subject = data.get("Subject");
                                    if subject.is_none() {
                                        return Err(AppError::BadRequest(format!(
                                            "subject nil: {:?}",
                                            data
                                        )));
                                    }

                                    let message = data.get("Message");
                                    if message.is_none() {
                                        return Err(AppError::BadRequest(format!(
                                            "message nil: {:?}",
                                            data
                                        )));
                                    }

                                    let subject = match subject.unwrap() {
                                        Value::String(sub) => sub.to_string().clone(),
                                        _ => "".to_string(),
                                    };

                                    let message = match message.unwrap() {
                                        Value::String(msg) => msg.to_string().clone(),
                                        _ => "".to_string(),
                                    };

                                    Ok((subject.parse().unwrap(), message))
                                }
                                _ => Err(AppError::BadRequest(format!("parse error: {:?}", data))),
                            }
                        }
                        _ => Err(AppError::BadRequest(format!("parse error: {:?}", data))),
                    }
                }
                _ => Err(AppError::BadRequest(format!("parse error: {:?}", data))),
            }
        }
        _ => Err(AppError::BadRequest(format!("parse error: {:?}", event))),
    }
}
