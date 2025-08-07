mod args;
mod error;
mod excel;
mod logger;
mod parser;
mod schema;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use calamine::{Reader, Xlsx, open_workbook};
use clap::Parser;
use polars::prelude::*;

use crate::{
    args::Args,
    excel::ToDataFrame,
    parser::parse_register,
    schema::base::{df_to_blks, df_to_compo, df_to_regs},
    schema::{ipxact, regvue},
};

fn main() -> anyhow::Result<(), error::Error> {
    logger::init();

    let args = Args::parse();
    let source = Path::new(&args.input);
    let mut wb: Xlsx<_> = open_workbook(source)?;
    let sheets = wb.worksheets();
    let mut df_map: HashMap<String, DataFrame> = sheets
        .iter()
        .map(|(sheet_name, range_data)| {
            range_data.to_data_frame().map(|df| (sheet_name.into(), df))
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    let compo = {

        let compo_df = df_map
            .remove("version")
            .ok_or_else(|| error::Error::NotFound("version".into()))?;

        df_to_compo(compo_df, || {

            let blks_df = df_map
                .remove("address_map")
                .ok_or_else(|| error::Error::NotFound("address_map".into()))?;

            df_to_blks(blks_df, |s| {

                tracing::debug!("block_name: {}", s);

                let regs_df = df_map
                    .remove(s)
                    .ok_or_else(|| error::Error::NotFound(s.into()))?;
                let parsered_df = parse_register(regs_df)?;

                df_to_regs(parsered_df)

            })
        })?
    };

    let ipxact_compo = ipxact::Component::try_from(&compo)?;
    let xml_str = quick_xml::se::to_string(&ipxact_compo)?;
    let xml_file = args
        .output
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(&source).with_extension("xml"));

    fs::write(&xml_file, xml_str)?;

    if args.regvue {
        let regvue_doc = regvue::Document::try_from(&compo)?;
        let json_str = serde_json::to_string_pretty(&regvue_doc)?;
        let json_file = xml_file.with_extension("json");

        fs::write(&json_file, json_str)?;
    }

    Ok(())
}
