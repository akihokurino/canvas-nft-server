#[derive(Clone, PartialEq)]
pub enum AuthUser {
    // 管理ユーザー
    Admin(String),
    // 一般ユーザー（ログイン済）
    Service(String),
    // 一般ユーザー（未ログイン）
    None,
}

impl AuthUser {
    pub fn user_id(&self) -> Option<String> {
        match self {
            AuthUser::Admin(id) => Some(id.to_owned()),
            AuthUser::Service(id) => Some(id.to_owned()),
            _ => None,
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
            AuthUser::Service(_id) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub address: String,
}

impl User {
    pub fn new(id: String, address: String) -> Self {
        Self { id, address }
    }

    pub fn with_balance(&self, balance: i32) -> UserWithBalance {
        UserWithBalance {
            id: self.id.to_owned(),
            address: self.address.to_owned(),
            balance,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UserWithBalance {
    pub id: String,
    pub address: String,
    pub balance: i32,
}
