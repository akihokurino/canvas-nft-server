use crate::domain::user::User;
use crate::ethereum::Client;
use crate::AppResult;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::types::U256;

impl Client {
    pub async fn mint_erc1155(&self, user: &User, work_id: String, amount: u32) -> AppResult<()> {
        let contract_address =
            env::var("NFT_1155_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address).unwrap(),
            include_bytes!("canvas_erc1155.abi.json"),
        )?;

        let prev_key = SecretKey::from_str(&user.wallet_secret).unwrap();
        let gas_limit: i64 = 5500000;
        let gas_price: i64 = 35000000000;

        let result = contract
            .signed_call_with_confirmations(
                "mint",
                (
                    self.parse_address(user.wallet_address.to_owned()).unwrap(),
                    work_id,
                    amount,
                ),
                Options::with(|opt| {
                    opt.gas = Some(U256::from(gas_limit));
                    opt.gas_price = Some(U256::from(gas_price));
                }),
                1,
                SecretKeyRef::from(&prev_key),
            )
            .await?;

        println!("tx id: {:?}", result.transaction_type);
        println!("gas used: {:?}", result.gas_used);
        println!("status: {:?}", result.status);

        Ok(())
    }
}
