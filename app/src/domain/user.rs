use crate::ethereum;

#[derive(Clone, PartialEq)]
pub enum AuthUser {
    // システム
    System,
    // NFT生成ユーザー
    Publisher(String),
    // 不明
    Unknown,
}

impl AuthUser {
    pub fn user_id(&self) -> Option<String> {
        match self {
            AuthUser::Publisher(id) => Some(id.to_owned()),
            _ => None,
        }
    }

    pub fn is_system(&self) -> bool {
        match self {
            AuthUser::System => true,
            _ => false,
        }
    }

    pub fn is_publisher(&self) -> bool {
        match self {
            AuthUser::Publisher(_id) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub wallet_address: String,
    pub wallet_secret: String,
}

impl User {
    pub fn new(id: String, address: String, secret: String) -> Self {
        Self {
            id,
            wallet_address: address,
            wallet_secret: secret,
        }
    }

    pub fn with_balance(
        &self,
        balance: u128,
        nft_721_num: u128,
        nft_1155_num: Vec<(String, u128)>,
    ) -> UserWithBalance {
        let amt_unit = "wei";
        let to_unit = "ether";
        let map = ethereum::unit::convert(format!("{}", balance).as_str(), &amt_unit);
        let val = map.get(to_unit).unwrap();

        UserWithBalance {
            id: self.id.to_owned(),
            wallet_address: self.wallet_address.to_owned(),
            balance: val.parse().unwrap_or(0.0),
            nft_721_num,
            nft_1155_num,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserWithBalance {
    pub id: String,
    pub wallet_address: String,
    pub balance: f64,
    pub nft_721_num: u128,
    pub nft_1155_num: Vec<(String, u128)>,
}
