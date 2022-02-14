pub mod api;

use std::env;
use web3::types::Address;
use web3::*;

#[derive(Clone, Debug)]
pub struct Client {
    cli: web3::Web3<transports::Http>,
}

impl Client {
    pub fn new() -> Self {
        let base_url = env::var("ETHEREUM_URL").expect("should set ethereum url");
        let transport = transports::Http::new(&base_url)
            .ok()
            .expect("should set ethereum url");
        let cli = Web3::new(transport);

        Client { cli }
    }

    pub fn parse_address(&self, address: String) -> Option<Address> {
        match address.trim_start_matches("0x").parse() {
            Ok(value) => Some(value),
            Err(_e) => None,
        }
    }
}
