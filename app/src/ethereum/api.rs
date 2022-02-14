use crate::ethereum::Client;
use crate::AppResult;

impl Client {
    pub async fn get_balance(&self, address: String) -> AppResult<i32> {
        let balance = self
            .cli
            .eth()
            .balance(self.parse_address(address).unwrap(), None)
            .await?;
        Ok(balance.as_u32() as i32)
    }
}
