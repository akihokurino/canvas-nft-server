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
    fn erc721(&self) -> AppResult<Contract<Http>> {
        let contract_address =
            env::var("ERC721_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address.clone()).unwrap(),
            include_bytes!("canvas_erc721.abi.json"),
        )?;
        Ok(contract)
    }

    pub async fn get_erc721_nft_balance(&self, user: &User) -> AppResult<u128> {
        let contract = self.erc721()?;
        let result = contract.query(
            "balanceOf",
            self.parse_address(user.wallet_address.to_owned()).unwrap(),
            None,
            Options::default(),
            None,
        );
        let balance_of: U256 = result.await?;

        Ok(balance_of.as_u128())
    }

    pub async fn get_erc721_used_names(&self) -> AppResult<Vec<String>> {
        let contract = self.erc721()?;
        let result = contract.query("usedTokenNames", (), None, Options::default(), None);
        let names: Vec<String> = result.await?;

        Ok(names)
    }

    pub async fn get_erc721_token_id_of(&self, work_id: String) -> AppResult<u128> {
        let contract = self.erc721()?;
        let result = contract.query("tokenIdOf", work_id, None, Options::default(), None);
        let id: u128 = result.await?;

        Ok(id)
    }

    pub async fn mint_erc721(
        &self,
        user: &User,
        work_id: String,
        ipfs_hash: String,
        s3_key: String,
    ) -> AppResult<()> {
        let contract = self.erc721()?;
        let prev_key = SecretKey::from_str(&user.wallet_secret).unwrap();
        let gas_limit: i64 = 5500000;
        let gas_price: i64 = 35000000000;

        let result = contract
            .signed_call_with_confirmations(
                "mint",
                (
                    self.parse_address(user.wallet_address.to_owned()).unwrap(),
                    work_id,
                    ipfs_hash,
                    s3_key,
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
