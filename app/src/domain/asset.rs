#[derive(Clone, Debug)]
pub struct Asset721 {
    pub work_id: String,
    pub contract_address: String,
    pub token_id: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub image_preview_url: String,
    pub permalink: String,
    pub usd_price: f64,
    pub eth_price: f64,
}

impl Asset721 {
    pub fn new(
        work_id: String,
        contract_address: String,
        token_id: String,
        name: String,
        description: String,
        image_url: String,
        image_preview_url: String,
        permalink: String,
    ) -> Self {
        Self {
            work_id,
            contract_address,
            token_id,
            name,
            description,
            image_url,
            image_preview_url,
            permalink,
            usd_price: 0.0,
            eth_price: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Asset1155 {
    pub work_id: String,
    pub contract_address: String,
    pub token_id: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub image_preview_url: String,
    pub permalink: String,
    pub usd_price: f64,
    pub eth_price: f64,
}

impl Asset1155 {
    pub fn new(
        work_id: String,
        contract_address: String,
        token_id: String,
        name: String,
        description: String,
        image_url: String,
        image_preview_url: String,
        permalink: String,
    ) -> Self {
        Self {
            work_id,
            contract_address,
            token_id,
            name,
            description,
            image_url,
            image_preview_url,
            permalink,
            usd_price: 0.0,
            eth_price: 0.0,
        }
    }
}
