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

    fn wallet_address(&self) -> String {
        self.data.wallet_address.to_owned()
    }

    fn balance(&self) -> f64 {
        self.data.balance.to_owned()
    }

    fn nft_721_num(&self) -> i32 {
        TryFrom::try_from(self.data.nft_721_num.to_owned()).unwrap_or_default()
    }

    fn nft_1155_num(&self) -> Vec<Nft1155Balance> {
        self.data
            .nft_1155_num
            .iter()
            .map(|v| Nft1155Balance {
                work_id: v.0.to_owned(),
                balance: TryFrom::try_from(v.1.to_owned()).unwrap_or_default(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, GraphQLObject)]
pub struct Nft1155Balance {
    work_id: String,
    balance: i32,
}

impl From<domain::user::UserWithBalance> for User {
    fn from(data: domain::user::UserWithBalance) -> Self {
        Self { data }
    }
}
