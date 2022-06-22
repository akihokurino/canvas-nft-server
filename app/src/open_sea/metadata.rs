use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub description: String,
    pub external_url: String,
    pub name: String,
    pub image: String,
}

impl Metadata {
    pub fn new(work_id: String, description: String, image_url: String) -> Self {
        Self {
            description,
            external_url: format!("https://canvas-329810.web.app/{}", work_id.clone()),
            name: work_id.clone(),
            image: image_url,
        }
    }
}
