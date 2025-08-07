use polars::error::PolarsError;
use polars::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    vendor: String,
    library: String,
    name: String,
    version: String,
    blks: Vec<Block>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    name: String,
    offset: String,
    range: String,
    size: String,
    regs: Vec<Register>,
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
    attr: String,
    reset: String,
    desc: String,
}

impl Component {
    pub fn vendor(&self) -> &str {
        &self.vendor
    }
    pub fn library(&self) -> &str {
        &self.library
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> &str {
        &self.version
    }
    pub fn blks(&self) -> &Vec<Block> {
        &self.blks
    }
}

impl Block {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn range(&self) -> &str {
        &self.range
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn regs(&self) -> &Vec<Register> {
        &self.regs
    }
}

impl Register {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn size(&self) -> &str {
        &self.size
    }
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}

impl Field {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn offset(&self) -> &str {
        &self.offset
    }
    pub fn width(&self) -> &str {
        &self.width
    }
    pub fn attr(&self) -> &str {
        &self.attr
    }
    pub fn reset(&self) -> &str {
        &self.reset
    }
    pub fn desc(&self) -> &str {
        &self.desc
    }
}

pub fn df_to_regs(df: &DataFrame) -> anyhow::Result<Vec<Register>, Error> {
    let extract_list =
        |df: &DataFrame, col_name: &str, idx: usize| -> anyhow::Result<Vec<String>, PolarsError> {
            df.column(col_name)?
                .list()?
                .get_as_series(idx)
                .ok_or_else(|| PolarsError::NoData("No data at index".into()))?
                .str()?
                .into_iter()
                .map(|opt_s| {
                    opt_s
                        .map(|s| s.into())
                        .ok_or_else(|| PolarsError::NoData("No data found in dataframe".into()))
                })
                .collect()
        };

    (0..df.height())
        .map(|i| {
            let name_array = extract_list(&df, "FIELD", i)?;
            let offset_array = extract_list(&df, "BIT_OFFSET", i)?;
            let width_array = extract_list(&df, "WIDTH", i)?;
            let attribute_array = extract_list(&df, "ATTRIBUTE", i)?;
            let default_array = extract_list(&df, "DEFAULT", i)?;
            let description_array = extract_list(&df, "DESCRIPTION", i)?;

            let fields = name_array
                .iter()
                .zip(offset_array.iter())
                .zip(width_array.iter())
                .zip(attribute_array.iter())
                .zip(default_array.iter())
                .zip(description_array.iter())
                .map(|(((((name, offset), width), attr), reset), desc)| Field {
                    name: name.into(),
                    offset: offset.into(),
                    width: width.into(),
                    attr: attr.into(),
                    reset: reset.into(),
                    desc: desc.into(),
                })
                .collect();

            let name = df
                .column("REG")?
                .str()?
                .get(i)
                .map(|s| s.into())
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?;
            let offset = df
                .column("ADDR")?
                .list()?
                .get_as_series(i)
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
                .str()?
                .get(0)
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
                .into();

            let size = df
                .column("REG_WIDTH")?
                .list()?
                .get_as_series(i)
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
                .str()?
                .get(0)
                .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
                .into();

            Ok(Register {
                name,
                offset,
                size,
                fields,
            })
        })
        .collect()
}

pub fn df_to_blks<F>(df: &DataFrame, registers_extractor: F) -> anyhow::Result<Vec<Block>, Error>
where
    F: Fn(&str) -> anyhow::Result<Vec<Register>, Error>,
{
    (0..df.height())
        .map(|i| {
            let name: String = df
                .column("BLOCK")?
                .str()?
                .get(i)
                .map(|s| s.into())
                .ok_or_else(|| Error::Polars(PolarsError::NoData("No data in DataFrame".into())))?;
            let offset: String = df
                .column("OFFSET")?
                .str()?
                .get(i)
                .map(|s| s.into())
                .ok_or_else(|| Error::Polars(PolarsError::NoData("No data in DataFrame".into())))?;
            let range: String = df
                .column("RANGE")?
                .str()?
                .get(i)
                .map(|s| s.into())
                .ok_or_else(|| Error::Polars(PolarsError::NoData("No data in DataFrame".into())))?;
            let size = "32".to_string();
            let regs = registers_extractor(&name)?;

            Ok(Block {
                name,
                offset,
                range,
                size,
                regs,
            })
        })
        .collect()
}

pub fn df_to_compo<F>(df: &DataFrame, blocks_extractor: F) -> anyhow::Result<Component, Error>
where
    F: Fn() -> anyhow::Result<Vec<Block>, Error>,
{
    let extract_tag = |df: &DataFrame, tag: &str| -> anyhow::Result<String, Error> {
        Ok(df
            .to_owned()
            .lazy()
            .filter(col("TAG").eq(lit(tag)))
            .collect()?
            .column("VALUE")?
            .str()?
            .get(0)
            .ok_or_else(|| PolarsError::NoData("No data in DataFrame".into()))?
            .into())
    };

    let vendor = extract_tag(&df, "VENDOR")?;
    let library = extract_tag(&df, "LIBRARY")?;
    let name = extract_tag(&df, "NAME")?;
    let version = extract_tag(&df, "VERSION")?;
    let blks = blocks_extractor()?;

    Ok(Component {
        vendor,
        library,
        name,
        version,
        blks,
    })
}
