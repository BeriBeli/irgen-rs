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
use crate::{
    excel::ToDataFrame,
    schema::base::{dataframe_to_blocks, dataframe_to_component},
};
//use crate::schema::ipxact;

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

    let component = dataframe_to_component(&(df_map.get("version").unwrap()), || {
        dataframe_to_blocks(&(df_map.get("address_map").unwrap()), |s| {
            tracing::debug!("block_name: {}", s);
            dataframe_to_registers(&parser_register(&(df_map.get(s).unwrap()))?)
        })
    })?;

    // tracing::info!("{:#?}", component);

    // let ipxact_component = ipxact::Component::from(&component);
    let pretty_json = serde_json::to_string_pretty(&component).unwrap();
    fs::write("person_pretty.json", pretty_json)?;

    Ok(())
}
