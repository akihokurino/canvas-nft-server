use crate::graph::Context;
use app::domain;
use core::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct User {
    data: domain::user::UserWithBalance,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    fn id(&self) -> String {
        self.data.id.to_owned()
    }

    fn address(&self) -> String {
        self.data.address.to_owned()
    }

    fn balance(&self) -> f64 {
        self.data.balance.to_owned()
    }

    fn nft_num(&self) -> i32 {
        TryFrom::try_from(self.data.nft_num.to_owned()).unwrap_or_default()
    }
}

impl From<domain::user::UserWithBalance> for User {
    fn from(data: domain::user::UserWithBalance) -> Self {
        Self { data }
    }
}
