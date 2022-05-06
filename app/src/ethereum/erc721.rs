use crate::domain::user::User;
use crate::ethereum::Client;
use crate::AppResult;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;
use web3::types::{Address, U256};

impl Client {
    pub async fn get_erc721_nft_name(&self) -> AppResult<String> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address).unwrap(),
            include_bytes!("canvas_erc721.abi.json"),
        )?;

        let result = contract.query("name", (), None, Options::default(), None);
        let name: String = result.await?;

        Ok(name)
    }

    pub async fn get_erc721_nft_symbol(&self) -> AppResult<String> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address).unwrap(),
            include_bytes!("canvas_erc721.abi.json"),
        )?;

        let result = contract.query("symbol", (), None, Options::default(), None);
        let symbol: String = result.await?;

        Ok(symbol)
    }

    pub async fn get_erc721_nft_balance(&self, user: &User) -> AppResult<u128> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address).unwrap(),
            include_bytes!("canvas_erc721.abi.json"),
        )?;

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

    pub async fn mint_erc721(&self, user: &User, work_id: String) -> AppResult<()> {
        let contract_address =
            env::var("NFT_721_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(contract_address).unwrap(),
            include_bytes!("canvas_erc721.abi.json"),
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
