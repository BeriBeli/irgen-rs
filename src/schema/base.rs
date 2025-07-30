use polars::prelude::*;
use polars::error::PolarsError;
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
    name: Option<String>,
    offset: Option<String>,
    size: Option<String>,
    fields: Option<Vec<Field>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    name: Option<String>,
    offset: Option<String>,
    width: Option<String>,
    attribute: Option<String>,
    default: Option<String>,
}

pub fn dataframe_to_registers(df: DataFrame) -> anyhow::Result<Vec<Register>, crate::error::Error> {

    let registers = (0..df.height()).map(|i| {
        let name_array: Vec<Option<String>> = df
            .column("FIELD")?
            .list()?
            .get_as_series(i)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_string()))
            .collect();
        let offset_array: Vec<Option<String>> = df
            .column("BIT_OFFSET")?
            .list()?
            .get_as_series(i)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_string()))
            .collect();
        let width_array: Vec<Option<String>> = df
            .column("WIDTH")?
            .list()?
            .get_as_series(i)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_string()))
            .collect();
        let attribute_array: Vec<Option<String>> = df
            .column("ATTRIBUTE")?
            .list()?
            .get_as_series(i)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_string()))
            .collect();
        let default_array: Vec<Option<String>> = df
            .column("DEFAULT")?
            .list()?
            .get_as_series(i)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_string()))
            .collect();
        let fields: Option<Vec<Field>> = (0..name_array.len()).map(
            |i| {
                let name = name_array[i].clone();
                let offset = offset_array[i].clone();
                let width = width_array[i].clone();
                let attribute = attribute_array[i].clone();
                let default = default_array[i].clone();
                Some(Field {name, offset, width, attribute, default})
            }
        ).collect();
        let name = df
            .column("REG")?
            .str()?
            .get(i)
            .map(|s| s.to_string());
        let offset = df
            .column("ADDR")?
            .list()?
            .get_as_series(i)
            .and_then(|s| s.str().map(|ca| ca.get(0).map(|s| s.to_string())).ok())
            .flatten();
        let size = df
            .column("REG_WIDTH")?
            .list()?
            .get_as_series(i)
            .and_then(|s| s.str().map(|ca| ca.get(0).map(|s| s.to_string())).ok())
            .flatten();
        Ok(Register {name, offset, size, fields})
    }).collect::<anyhow::Result<Vec<Register>>>()?;

    Ok(registers)

}

pub fn dataframe_to_blocks(df: DataFrame) -> anyhow::Result<Vec<Block>, crate::error::Error> {
    todo!()
}

pub fn dataframe_to_component(df: DataFrame) -> anyhow::Result<Component, crate::error::Error> {
    todo!()
}

// PolarsError
