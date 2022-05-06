use crate::graph::Context;
use app::domain;

#[derive(Debug, Clone)]
pub struct Asset {
    data: domain::asset::Asset721,
}

#[juniper::graphql_object(Context = Context)]
impl Asset {
    fn work_id(&self) -> String {
        self.data.work_id.to_owned()
    }

    fn address(&self) -> String {
        self.data.contract_address.to_owned()
    }

    fn token_id(&self) -> String {
        self.data.token_id.to_owned()
    }

    fn name(&self) -> String {
        self.data.name.to_owned()
    }

    fn description(&self) -> String {
        self.data.description.to_owned()
    }

    fn image_url(&self) -> String {
        self.data.image_url.to_owned()
    }

    fn image_preview_url(&self) -> String {
        self.data.image_preview_url.to_owned()
    }

    fn permalink(&self) -> String {
        self.data.permalink.to_owned()
    }

    fn usd_price(&self) -> f64 {
        self.data.usd_price.to_owned()
    }

    fn eth_price(&self) -> f64 {
        self.data.eth_price.to_owned()
    }
}

impl From<domain::asset::Asset721> for Asset {
    fn from(data: domain::asset::Asset721) -> Self {
        Self { data }
    }
}
