use crate::domain::user::AuthUser;
use crate::{AppError, AppResult};
use aws_sdk_cognitoidentityprovider::model::AttributeType;
use aws_sdk_cognitoidentityprovider::Client;
use jsonwebtoken::{DecodingKey, Validation};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Verifier {
    user_pool_id: String,
    jwks: Arc<RwLock<HashMap<String, Jwk>>>,
}

impl Verifier {
    pub fn new() -> Self {
        let user_pool_id = env::var("COGNITO_USER_POOL_ID").expect("need set cognito pool id");

        Verifier {
            user_pool_id,
            jwks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn verify_token(&self, token: &str) -> AppResult<AuthUser> {
        let token_header =
            jsonwebtoken::decode_header(token).map_err(|_err| AppError::UnAuthenticate)?;
        let kid = token_header.kid.ok_or_else(|| AppError::UnAuthenticate)?;
        let jwk = self.jwks.read().unwrap().get(&kid).cloned();
        let jwk = match jwk {
            None => {
                self.refresh_jwks()
                    .await
                    .map_err(|err| AppError::Internal(err.to_string()))?;
                let jwk = self.jwks.read().unwrap().get(&kid).cloned();
                match jwk {
                    None => return Err(AppError::UnAuthenticate),
                    Some(v) => v,
                }
            }
            Some(v) => v,
        };

        let token = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_rsa_components(&jwk.n, &jwk.e)
                .map_err(|_err| AppError::UnAuthenticate)?,
            &Validation::new(jsonwebtoken::Algorithm::RS256),
        )
        .map_err(|_err| AppError::UnAuthenticate)?;

        let sub = token.claims.sub.ok_or_else(|| AppError::UnAuthenticate)?;
        let raw_account_type = token
            .claims
            .account_type
            .ok_or_else(|| AppError::UnAuthenticate)?;

        Ok(if raw_account_type.to_string() == "0".to_string() {
            AuthUser::Publisher(sub)
        } else {
            AuthUser::Unknown
        })
    }

    async fn refresh_jwks(&self) -> Result<(), String> {
        let jwks = self.fetch_jwks().await?;
        *self.jwks.write().unwrap() = jwks;
        Ok(())
    }

    async fn fetch_jwks(&self) -> Result<HashMap<String, Jwk>, String> {
        Ok(reqwest::get(format!(
            "https://cognito-idp.ap-northeast-1.amazonaws.com/{}/.well-known/jwks.json",
            self.user_pool_id
        ))
        .await
        .map_err(|err| err.to_string())?
        .json::<KeyResponse>()
        .await
        .map_err(|err| err.to_string())?
        .keys
        .into_iter()
        .map(|k| (k.kid.clone(), k))
        .collect())
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Jwk {
    e: String,
    kid: String,
    n: String,
}

#[derive(Debug, Deserialize)]
struct KeyResponse {
    keys: Vec<Jwk>,
}

#[derive(Debug, Clone, Deserialize)]
struct Claims {
    #[serde(rename = "sub")]
    sub: Option<String>,
    #[serde(rename = "custom:account_type")]
    account_type: Option<String>,
}

pub async fn get_email(id: String) -> AppResult<String> {
    let pool_id = env::var("COGNITO_USER_POOL_ID").expect("need set cognito pool id");

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

pub async fn create_user(email: String, password: String) -> AppResult<String> {
    let pool_id = env::var("COGNITO_USER_POOL_ID").expect("need set cognito pool id");

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let attr = AttributeType::builder()
        .set_name(Some("custom:account_type".to_string()))
        .set_value(Some("0".to_string()))
        .build();

    let response = client
        .admin_create_user()
        .user_pool_id(pool_id.clone())
        .username(email.to_string())
        .user_attributes(attr)
        .send()
        .await?;

    client
        .admin_set_user_password()
        .user_pool_id(pool_id.clone())
        .username(email.to_string())
        .password(password.to_string())
        .permanent(true)
        .send()
        .await?;

    Ok(response.user.unwrap().username.unwrap())
}
