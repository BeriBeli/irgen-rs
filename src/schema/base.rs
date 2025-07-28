use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    vendor: String,
    library: String,
    name: String,
    version: String,
    blocks: Vec<Block>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: String,
    offset: String,
    range: String,
    size: String,
    registers: Vec<Register>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    name: String,
    offset: String,
    size: String,
    fields: Vec<Field>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: String,
    offset: String,
    width: String,
    attribute: String,
    default: String,
}