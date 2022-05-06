use crate::aws::cognito;
use crate::ddb::Dao;
use crate::domain::user::{User, UserWithBalance};
use crate::{ethereum, AppError, AppResult};

pub struct Application {
    #[allow(dead_code)]
    me_id: String,
    user_dao: Dao<User>,
    ethereum_cli: ethereum::Client,
}

impl Application {
    pub async fn new(me_id: String) -> Self {
        let user_dao: Dao<User> = Dao::new().await;
        let ethereum_cli = ethereum::Client::new();

        Self {
            me_id,
            user_dao,
            ethereum_cli,
        }
    }

    pub async fn get_me(&self) -> AppResult<UserWithBalance> {
        let user = self.user_dao.get(self.me_id.clone()).await?;
        let balance = self
            .ethereum_cli
            .get_balance(user.wallet_address.clone())
            .await?;
        let nft_balance = self
            .ethereum_cli
            .get_erc721_nft_balance(user.wallet_address.clone())
            .await?;

        Ok(user.with_balance(balance, nft_balance))
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        wallet_address: String,
        wallet_secret: String,
    ) -> AppResult<String> {
        let user = self
            .user_dao
            .get_by_wallet_address(wallet_address.clone())
            .await;
        if let Ok(_user) = user {
            return Err(AppError::BadRequest(
                "すでにユーザーが存在します".to_string(),
            ));
        }
        if let Err(err) = user {
            if err != AppError::NotFound {
                return Err(err);
            }
        }

        let user_id = cognito::create_user(email, password).await?;

        let user = User::new(user_id.clone(), wallet_address, wallet_secret);
        self.user_dao.put(&user).await?;

        Ok(user_id.to_owned())
    }
}
