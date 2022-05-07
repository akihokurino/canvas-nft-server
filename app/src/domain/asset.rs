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
    pub fn new(work_id: String) -> Self {
        Self {
            work_id,
            contract_address: "".to_string(),
            token_id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            image_url: "".to_string(),
            image_preview_url: "".to_string(),
            permalink: "".to_string(),
            usd_price: 0.0,
            eth_price: 0.0,
        }
    }

    pub fn published(
        &mut self,
        contract_address: String,
        token_id: String,
        name: String,
        description: String,
        image_url: String,
        image_preview_url: String,
        permalink: String,
    ) {
        self.contract_address = contract_address;
        self.token_id = token_id;
        self.name = name;
        self.description = description;
        self.image_url = image_url;
        self.image_preview_url = image_preview_url;
        self.permalink = permalink;
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
    pub fn new(work_id: String) -> Self {
        Self {
            work_id,
            contract_address: "".to_string(),
            token_id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            image_url: "".to_string(),
            image_preview_url: "".to_string(),
            permalink: "".to_string(),
            usd_price: 0.0,
            eth_price: 0.0,
        }
    }

    pub fn published(
        &mut self,
        contract_address: String,
        token_id: String,
        name: String,
        description: String,
        image_url: String,
        image_preview_url: String,
        permalink: String,
    ) {
        self.contract_address = contract_address;
        self.token_id = token_id;
        self.name = name;
        self.description = description;
        self.image_url = image_url;
        self.image_preview_url = image_preview_url;
        self.permalink = permalink;
    }
}
