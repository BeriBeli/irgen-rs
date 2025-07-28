use polars::prelude::*;

pub fn get_value_by_tag(df: &DataFrame, tag: &str) -> anyhow::Result<String> {
    let result_df = df
        .clone()
        .lazy()
        .filter(col("TAG").eq(lit(tag)))
        .select(&[col("VALUE")])
        .collect()?;

    let column = result_df.column("VALUE")?;

    let chunked_array = column.str()?;

    let result_vec: Vec<String> = chunked_array
        .into_iter()
        .flatten() // Filters out any null/None values
        .map(|s| s.to_string()) // Converts each &str into an owned String
        .collect();

    Ok(result_vec.first().unwrap().clone())
}