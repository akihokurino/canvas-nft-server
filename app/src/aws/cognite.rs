use crate::{AppError, AppResult};
use aws_sdk_cognitoidentityprovider::Client;
use jsonwebtokens_cognito::KeySet;
use std::env;

pub async fn verify_token(token: &str) -> AppResult<String> {
    let pool_id = env::var("COGNITE_USER_POOL_ID").expect("need set cognite pool id");
    let client_id = env::var("COGNITE_CLIENT_ID").expect("need set cognite client id");

    let keyset = KeySet::new("ap-northeast-1", pool_id)
        .map_err(|_err| AppError::Internal("".to_string()))?;
    let verifier = keyset
        .new_id_token_verifier(&[&client_id])
        .build()
        .map_err(|_err| AppError::Internal("".to_string()))?;

    let result = keyset
        .verify(token, &verifier)
        .await
        .map_err(|_err| AppError::UnAuthenticate)?;

    Ok(result.get("sub").map_or("".to_string(), |v| v.to_string()))
}

pub async fn get_email(id: String) -> AppResult<String> {
    let pool_id = env::var("COGNITE_USER_POOL_ID").expect("need set cognite pool id");

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let response = client
        .admin_get_user()
        .user_pool_id(pool_id)
        .username(id)
        .send()
        .await?;

    let attrs = response.user_attributes.unwrap();
    let mut email: String = "".to_string();
    for attr in attrs {
        if attr.name.unwrap() == "email" {
            email = attr.value.unwrap_or("".to_string())
        }
    }

    Ok(email)
}
