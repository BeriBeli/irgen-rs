use polars::error::PolarsError;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    vendor: Option<String>,
    library: Option<String>,
    name: Option<String>,
    version: Option<String>,
    blocks: Option<Vec<Block>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: Option<String>,
    offset: Option<String>,
    range: Option<String>,
    size: Option<String>,
    registers: Option<Vec<Register>>,
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

impl Component {
    pub fn vendor(&self) -> &Option<String> {
        &self.vendor
    }
    pub fn library(&self) -> &Option<String> {
        &self.library
    }
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn version(&self) -> &Option<String> {
        &self.version
    }
    pub fn blocks(&self) -> &Option<Vec<Block>> {
        &self.blocks
    }
}

impl Block {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn offset(&self) -> &Option<String> {
        &self.offset
    }
    pub fn range(&self) -> &Option<String> {
        &self.range
    }
    pub fn size(&self) -> &Option<String> {
        &self.size
    }
    pub fn registers(&self) -> &Option<Vec<Register>> {
        &self.registers
    }
}

impl Register {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn offset(&self) -> &Option<String> {
        &self.offset
    }
    pub fn size(&self) -> &Option<String> {
        &self.size
    }
    pub fn fields(&self) -> &Option<Vec<Field>> {
        &self.fields
    }
}

impl Field {
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
    pub fn offset(&self) -> &Option<String> {
        &self.offset
    }
    pub fn width(&self) -> &Option<String> {
        &self.width
    }
    pub fn attribute(&self) -> &Option<String> {
        &self.attribute
    }
    pub fn default(&self) -> &Option<String> {
        &self.default
    }
}

pub fn dataframe_to_registers(
    df: &DataFrame,
) -> anyhow::Result<Vec<Register>, crate::error::Error> {
    let extract_list = |df: &DataFrame,
                        col_name: &str,
                        idx: usize|
     -> anyhow::Result<Vec<Option<String>>, crate::error::Error> {
        Ok(df
            .column(col_name)?
            .list()?
            .get_as_series(idx)
            .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
            .str()?
            .into_iter()
            .map(|opt_s| opt_s.map(|s| s.to_owned()))
            .collect())
    };

    (0..df.height())
        .map(|i| {
            let name_array = extract_list(&df, "FIELD", i)?;
            let offset_array = extract_list(&df, "BIT_OFFSET", i)?;
            let width_array = extract_list(&df, "WIDTH", i)?;
            let attribute_array = extract_list(&df, "ATTRIBUTE", i)?;
            let default_array = extract_list(&df, "DEFAULT", i)?;

            let fields = Some(
                name_array
                    .iter()
                    .zip(offset_array.iter())
                    .zip(width_array.iter())
                    .zip(attribute_array.iter())
                    .zip(default_array.iter())
                    .map(|((((name, offset), width), attribute), default)| Field {
                        name: name.clone(),
                        offset: offset.clone(),
                        width: width.clone(),
                        attribute: attribute.clone(),
                        default: default.clone(),
                    })
                    .collect(),
            );

            let name = df.column("REG")?.str()?.get(i).map(|s| s.to_owned());
            let offset = df
                .column("ADDR")?
                .list()?
                .get_as_series(i)
                .and_then(|s| s.str().map(|ca| ca.get(0).map(|s| s.to_owned())).ok())
                .flatten();
            let size = df
                .column("REG_WIDTH")?
                .list()?
                .get_as_series(i)
                .and_then(|s| s.i32().map(|ca| ca.get(0).map(|s| s.to_string())).ok())
                .flatten();

            Ok(Register {
                name,
                offset,
                size,
                fields,
            })
        })
        .collect()
}

pub fn dataframe_to_blocks<F>(
    df: &DataFrame,
    registers_extractor: F,
) -> anyhow::Result<Vec<Block>, crate::error::Error>
where
    F: Fn(&str) -> anyhow::Result<Vec<Register>, crate::error::Error>,
{
    (0..df.height())
        .map(|i| {
            let name = df.column("BLOCK")?.str()?.get(i).map(|s| s.to_owned());
            let block_name = name
                .as_ref()
                .ok_or_else(|| PolarsError::NoData("Block name not found".into()))?;
            let offset = df.column("OFFSET")?.str()?.get(i).map(|s| s.to_owned());
            let range = df.column("RANGE")?.str()?.get(i).map(|s| s.to_owned());
            let size = Some("32".to_owned());
            let registers = Some(registers_extractor(block_name)?);

            Ok(Block {
                name,
                offset,
                range,
                size,
                registers,
            })
        })
        .collect()
}

pub fn dataframe_to_component<F>(
    df: &DataFrame,
    blocks_extractor: F,
) -> anyhow::Result<Component, crate::error::Error>
where
    F: Fn() -> anyhow::Result<Vec<Block>, crate::error::Error>,
{
    let extract_df_tag =
        |df: &DataFrame, tag: &str| -> anyhow::Result<String, crate::error::Error> {
            Ok(df
                .clone()
                .lazy()
                .filter(col("TAG").eq(lit(tag)))
                .collect()?
                .column("VALUE")?
                .str()?
                .get(0)
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
                .to_owned())
        };

    let vendor = Some(extract_df_tag(&df, "VENDOR")?);
    let library = Some(extract_df_tag(&df, "LIBRARY")?);
    let name = Some(extract_df_tag(&df, "NAME")?);
    let version = Some(extract_df_tag(&df, "VERSION")?);
    let blocks = Some(blocks_extractor()?);

    Ok(Component {
        vendor,
        library,
        name,
        version,
        blocks,
    })
}
