mod logger;
mod error;
mod schema;
mod parser;

// use calamine::{Reader, Xlsx, open_workbook};
// use polars::prelude::*;

fn main() -> anyhow::Result<(), error::Error> {
    logger::init();

    let source = "example.xlsx".to_string();

    // let df = excel::read_excel(&source, "block0")?;

    // println!("{:#?}", df);

    // let mut workbook: Xlsx<_> = open_workbook(&source)?;

    // let sheet_names = workbook.sheet_names().to_owned();
        
    // println!("{:#?}", sheet_names);

    // let sheets = workbook.worksheets();

    // for (sheet_name, range_data) in sheets.iter() {
    //     println!("sheet_name: {}", sheet_name);
    //     println!("{:#?}", range_data);
    // }

    // let parserd_df = parser::parser_register(&df)?;

    // println!("{:#?}", parserd_df);

    Ok(())
}
