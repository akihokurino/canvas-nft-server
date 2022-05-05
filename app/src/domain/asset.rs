#[derive(Clone, Debug)]
pub struct Asset {
    pub work_id: String,
    pub address: String,
    pub token_id: String,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub image_preview_url: String,
    pub permalink: String,
    pub usd_price: f64,
    pub eth_price: f64,
}

impl Asset {
    pub fn new(
        work_id: String,
        address: String,
        token_id: String,
        name: String,
        description: String,
        image_url: String,
        image_preview_url: String,
        permalink: String,
        usd_price: f64,
        eth_price: f64,
    ) -> Self {
        Self {
            work_id,
            address,
            token_id,
            name,
            description,
            image_url,
            image_preview_url,
            permalink,
            usd_price,
            eth_price,
        }
    }
}
