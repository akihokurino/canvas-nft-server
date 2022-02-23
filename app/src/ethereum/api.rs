use crate::ethereum::Client;
use crate::AppResult;
use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::signing::SecretKeyRef;

impl Client {
    pub async fn get_balance(&self, address: String) -> AppResult<u128> {
        let balance = self
            .cli
            .eth()
            .balance(self.parse_address(address).unwrap(), None)
            .await?;
        Ok(balance.as_u128())
    }

    pub async fn get_nft_name(&self) -> AppResult<String> {
        let address = env::var("NFT_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(address).unwrap(),
            include_bytes!("./canvas.abi.json"),
        )?;

        let result = contract.query("name", (), None, Options::default(), None);
        let name: String = result.await?;

        Ok(name)
    }

    pub async fn get_nft_symbol(&self) -> AppResult<String> {
        let address = env::var("NFT_CONTRACT_ADDRESS").expect("should set contract address");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(address).unwrap(),
            include_bytes!("./canvas.abi.json"),
        )?;

        let result = contract.query("symbol", (), None, Options::default(), None);
        let symbol: String = result.await?;

        Ok(symbol)
    }

    pub async fn mint_nft(&self, work_id: String) -> AppResult<()> {
        let address = env::var("NFT_CONTRACT_ADDRESS").expect("should set contract address");
        let wallet_address = env::var("MY_WALLET_ADDRESS").expect("should set wallet address");
        let wallet_secret = env::var("MY_WALLET_SECRET").expect("should set wallet secret");
        let contract = Contract::from_json(
            self.cli.eth(),
            self.parse_address(address).unwrap(),
            include_bytes!("./canvas.abi.json"),
        )?;

        let prev_key = SecretKey::from_str(&wallet_secret).unwrap();

        contract
            .signed_call_with_confirmations(
                "mint",
                (self.parse_address(wallet_address).unwrap(), work_id),
                Options::default(),
                1,
                SecretKeyRef::from(&prev_key),
            )
            .await?;

        Ok(())
    }
}
