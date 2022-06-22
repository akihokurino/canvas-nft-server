use crate::domain::user::User;
use crate::ethereum::Client;
use crate::AppResult;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::transports::Http;
use web3::types::U256;

impl Client {
    fn erc1155(&self) -> AppResult<Contract<Http>> {
        let contract_address =
            env::var("ERC1155_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address.clone()).unwrap(),
            include_bytes!("canvas_erc1155.abi.json"),
        )?;
        Ok(contract)
    }

    pub async fn get_erc1155_nft_balance(&self, user: &User) -> AppResult<Vec<(String, u128)>> {
        let contract = self.erc1155()?;

        let result = contract.query("usedTokenNames", (), None, Options::default(), None);
        let names: Vec<String> = result.await?;

        let mut balances: Vec<(String, u128)> = vec![];
        for name in names {
            let result = contract.query("tokenIdOf", name.clone(), None, Options::default(), None);
            let token_id: u128 = result.await?;

            let result = contract.query(
                "balanceOf",
                (
                    self.parse_address(user.wallet_address.to_owned()).unwrap(),
                    token_id,
                ),
                None,
                Options::default(),
                None,
            );
            let balance_of: U256 = result.await?;
            balances.push((name.to_owned(), balance_of.to_owned().as_u128()))
        }

        Ok(balances)
    }

    pub async fn get_erc1155_used_names(&self) -> AppResult<Vec<String>> {
        let contract = self.erc1155()?;
        let result = contract.query("usedTokenNames", (), None, Options::default(), None);
        let names: Vec<String> = result.await?;

        Ok(names)
    }

    pub async fn get_erc1155_token_id_of(&self, work_id: String) -> AppResult<u128> {
        let contract = self.erc1155()?;
        let result = contract.query("tokenIdOf", work_id, None, Options::default(), None);
        let id: u128 = result.await?;

        Ok(id)
    }

    pub async fn mint_erc1155(&self, user: &User, work_id: String, amount: u32) -> AppResult<()> {
        let contract = self.erc1155()?;
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
