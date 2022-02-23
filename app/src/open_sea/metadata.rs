use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub description: String,
    pub external_url: String,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub image: String,
}

impl Metadata {
    pub fn new(
        work_id: String,
        description: String,
        image_url: String,
        point: i32,
        level: i32,
    ) -> Self {
        let a1 = Attribute {
            trait_type: "Point".to_string(),
            display_type: "number".to_string(),
            value: point,
        };
        let a2 = Attribute {
            trait_type: "Level".to_string(),
            display_type: "number".to_string(),
            value: level,
        };

        Self {
            description,
            external_url: format!("https://canvas-329810.web.app/{}", work_id.clone()),
            name: work_id.clone(),
            attributes: vec![a1, a2],
            image: image_url,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub display_type: String,
    pub value: i32,
}
