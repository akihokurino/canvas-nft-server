use crate::graph::Context;
use app::domain;

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
}

impl From<domain::user::UserWithBalance> for User {
    fn from(data: domain::user::UserWithBalance) -> Self {
        Self { data }
    }
}
