use app::application::*;
use app::aws::*;
use app::{AppError, AppResult};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    ssm::load_env().await;
    lambda_runtime::run(handler_fn(exec)).await?;
    Ok(())
}

async fn exec(event: Value, _: Context) -> Result<(), Error> {
    let command = get_command_from_batch_event(event)
        .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
    println!("exec {:?}", command);

    if command == "sync-work" {
        let admin_work_app = admin::work::Application::new("batch".to_string()).await;
        admin_work_app
            .sync_work_thumbnail()
            .await
            .map_err(|e| simple_error::SimpleError::new(format!("error: {:?}", e)))?;
    }

    Ok(())
}

fn get_command_from_batch_event(event: Value) -> AppResult<String> {
    match event {
        Value::Object(data) => {
            let command = data.get("command");
            if command.is_none() {
                return Err(AppError::BadRequest(format!("command nil: {:?}", data)));
            }
            match command.unwrap() {
                Value::String(val) => Ok(val.to_string().clone()),
                _ => Err(AppError::BadRequest(format!("command empty: {:?}", data))),
            }
        }
        _ => Err(AppError::BadRequest(format!("parse error: {:?}", event))),
    }
}
