mod logger;
mod error;
mod schema;
mod parser;
mod excel;

// use calamine::{open_workbook, Reader, Xlsx};
use crate::excel::ToDataFrame;
// use polars::prelude::*;

fn main() -> anyhow::Result<(), error::Error> {
    logger::init();

    let source = "example.xlsx".to_string();

    let df = excel::ToDataFrameReader::new(&source)
        .open_sheet("block0")
        .ok_or("failed to open sheet")?
        .to_data_frame()?;

    println!("{:#?}", df);

    let parserd_df = parser::parser_register(&df)?;

    println!("{:#?}", parserd_df);

    let registers = schema::base::dataframe_to_registers(parserd_df)?;

    println!("{:#?}", registers);

    Ok(())
}
