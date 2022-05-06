use crate::domain::user::User;
use crate::ethereum::Client;
use crate::AppResult;

impl Client {
    pub async fn get_balance(&self, user: &User) -> AppResult<u128> {
        let balance = self
            .cli
            .eth()
            .balance(
                self.parse_address(user.wallet_address.to_owned()).unwrap(),
                None,
            )
            .await?;
        Ok(balance.as_u128())
    }
}
