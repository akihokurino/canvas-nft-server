use crate::ethereum;

#[derive(Clone, PartialEq)]
pub enum AuthUser {
    // マスターユーザー
    Master,
    // 管理ユーザー
    Admin(String),
    // 一般ユーザー（ログイン済）
    User(String),
    // 一般ユーザー（未ログイン）
    None,
}

impl AuthUser {
    pub fn user_id(&self) -> Option<String> {
        match self {
            AuthUser::Admin(id) => Some(id.to_owned()),
            AuthUser::User(id) => Some(id.to_owned()),
            _ => None,
        }
    }

    pub fn is_master(&self) -> bool {
        match self {
            AuthUser::Master => true,
            _ => false,
        }
    }

    pub fn is_admin(&self) -> bool {
        match self {
            AuthUser::Admin(_id) => true,
            _ => false,
        }
    }

    pub fn is_service(&self) -> bool {
        match self {
            AuthUser::User(_id) => true,
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

    pub fn with_balance(&self, balance: u128, nft_721_num: u128) -> UserWithBalance {
        let amt_unit = "wei";
        let to_unit = "ether";
        let map = ethereum::unit::convert(format!("{}", balance).as_str(), &amt_unit);
        let val = map.get(to_unit).unwrap();

        UserWithBalance {
            id: self.id.to_owned(),
            wallet_address: self.wallet_address.to_owned(),
            balance: val.parse().unwrap_or(0.0),
            nft_721_num,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserWithBalance {
    pub id: String,
    pub wallet_address: String,
    pub balance: f64,
    pub nft_721_num: u128,
}
