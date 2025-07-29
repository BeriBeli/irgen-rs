use polars::prelude::*;
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
    name: Option<String>,
    offset: Option<String>,
    width: Option<String>,
    attribute: Option<String>,
    default: Option<String>,
}

pub fn dataframe_to_fields(df: DataFrame) -> anyhow::Result<Vec<Field>> {

    let fields = (0..df.height()).map(|i| {
        let name = df.column("FIELD")?.str()?.get(i).map(|s| s.to_string());
        let offset = df.column("BIT")?.str()?.get(i).map(|s| s.to_string());
        let width = df.column("WIDTH")?.str()?.get(i).map(|s| s.to_string());
        let attribute = df.column("ATTRIBUTE")?.str()?.get(i).map(|s| s.to_string());
        let default = df.column("DEFAULT")?.str()?.get(i).map(|s| s.to_string());
        Ok(Field {name, offset, width, attribute, default})
    }).collect::<anyhow::Result<Vec<Field>>>()?;

    Ok(fields)
}