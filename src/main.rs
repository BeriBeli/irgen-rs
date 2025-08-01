mod error;
mod excel;
mod logger;
mod parser;
mod schema;

use std::collections::HashMap;
use std::fs;

use calamine::{Reader, Xlsx, open_workbook};
use polars::prelude::*;

use crate::parser::parser_register;
use crate::schema::base::dataframe_to_registers;
use crate::schema::ipxact;
use crate::{
    excel::ToDataFrame,
    schema::base::{dataframe_to_blocks, dataframe_to_component},
};

fn main() -> anyhow::Result<(), error::Error> {
    logger::init();

    let source = "example.xlsx".to_string();

    let mut wb: Xlsx<_> = open_workbook(&source)?;

    let sheets = wb.worksheets();

    let df_map: HashMap<String, DataFrame> = sheets
        .iter()
        .map(|(sheet_name, range_data)| {
            range_data
                .to_data_frame()
                .map(|df| (sheet_name.to_owned(), df))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let component = {
        let component_df = df_map
            .get("version")
            .ok_or_else(|| error::Error::NotFound("version".into()))?;
        dataframe_to_component(component_df, || {
            let blocks_df = df_map
                .get("address_map")
                .ok_or_else(|| error::Error::NotFound("address_map".into()))?;
            dataframe_to_blocks(blocks_df, |s| {
                tracing::debug!("block_name: {}", s);
                let register_df = df_map
                    .get(s)
                    .ok_or_else(|| error::Error::NotFound(s.into()))?;
                let parsered_df = parser_register(register_df)?;
                dataframe_to_registers(&parsered_df)
            })
        })?
    };

    // tracing::info!("{:#?}", component);

    let ipxact_component = ipxact::Component::from(&component)?;

    let xml = quick_xml::se::to_string(&ipxact_component)?;
    let json = serde_json::to_string_pretty(&ipxact_component)?;

    fs::write("example.xml", xml)?;
    fs::write("example.json", json)?;

    Ok(())
}
