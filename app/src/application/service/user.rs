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
        let balance = self.ethereum_cli.get_balance(user.address.clone()).await?;

        Ok(user.with_balance(balance))
    }

    pub async fn register(&self, address: String) -> AppResult<UserWithBalance> {
        println!("id: {:?}", self.me_id.clone());
        println!("address: {:?}", address.clone());
        let user = self.user_dao.get(self.me_id.clone()).await;
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

        let user = User::new(self.me_id.clone(), address);
        self.user_dao.put(&user).await?;

        let balance = self.ethereum_cli.get_balance(user.address.clone()).await?;

        Ok(user.with_balance(balance))
    }
}
