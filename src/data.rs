use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "vehicle")]
pub struct Vehicle {
    pub bodies: Bodies,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "bodies")]
pub struct Bodies {
    #[serde(rename = "body")]
    pub bodies: Vec<Body>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "body")]
pub struct Body {
    pub components: Components,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "components")]
pub struct Components {
    #[serde(rename = "c")]
    pub components: Vec<Component>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "c")]
pub struct Component {
    #[serde(rename = "@d", default = "default_definition")]
    pub definition: String,

    #[serde(rename = "o")]
    pub origin: Origin,
}

fn default_definition() -> String {
    "01_block".to_owned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "o")]
pub struct Origin {
    #[serde(default)]
    pub microprocessor_definition: Option<MicrocontrollerDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "microprocessor_definition")]
pub struct MicrocontrollerDef {
    #[serde(rename = "@width")]
    pub width: u8,
    #[serde(rename = "@length")]
    pub length: u8,
}

// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "definition")]
pub struct Definition {
    #[serde(
        rename = "@value",
        deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string",
        default
    )]
    pub value: u32,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(
        rename = "@mass",
        deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string"
    )]
    pub mass: f32,
}
