mod logger;
mod error;
mod schema;
mod parser;
mod excel;

use calamine::{open_workbook, Reader, Xlsx};
use crate::excel::ToDataFrame;
// use polars::prelude::*;

fn main() -> anyhow::Result<(), error::Error> {
    logger::init();

    let source = "example.xlsx".to_string();

    // let mut workbook: Xlsx<_> = open_workbook(&source)?;

    // let sheet_names = workbook.sheet_names().to_owned();

    // println!("{:#?}", sheet_names);

    // let sheets = workbook.worksheets();

    // for (sheet_name, range_data) in sheets.iter() {
    //     println!("sheet_name: {}", sheet_name);
    //     println!("{:#?}", range_data);
    // }

    let df = excel::ToDataFrameReader::new(&source)
        .open_sheet("block0")
        .ok_or("failed to open sheet")?
        .to_data_frame()?;

    println!("{:#?}", df);

    let parserd_df = parser::parser_register(&df)?;

    println!("{:#?}", parserd_df);

    Ok(())
}
