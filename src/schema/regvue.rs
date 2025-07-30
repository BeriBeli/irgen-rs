use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct  Schema {
    name: Option<String>, // register-description-format
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    text: Option<String>,
    href: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Root {
    desc: Option<String>,
    version: Option<String>,
    links: Option<Vec<Link>>,
    children: Option<Vec<Schema>>,
    expanded: Option<Vec<String>>,
    data_width: Option<i32>,
    default_reset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnumValue {
    name: Option<String>,
    value: Option<String>,
    doc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reset {
    value: Option<String>,
    names: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResetType {
    Reset(Reset),
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Field {
    name: Option<String>,
    nbits: Option<i32>,
    lsb: Option<i32>,
    access: Option<String>,
    reset: Option<ResetType>,
    doc: Option<String>,
    enum_: Option<EnumValue>,
    // experimental
    repr: Option<String>,
    custom_decode: Option<String>,
    custom_encode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Element {
    type_: Option<String>,
    id_: Option<String>,
    version: Option<String>,
    name: Option<String>,
    offset: Option<String>,
    size: Option<String>,
    links: Option<Vec<Link>>,
    children: Option<Vec<String>>,
    url: Option<String>,
    desc: Option<String>,
    fields: Option<Vec<Field>>,
    doc: Option<String>,
    data_width: Option<i32>,
    default_reset: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    schema_: Option<Schema>,
    root: Option<Root>,
    elements: Option<HashMap<String, Element>>,
}