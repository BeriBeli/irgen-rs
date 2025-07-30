use polars::prelude::*;

pub fn get_value_by_tag(df: &DataFrame, tag: &str) -> anyhow::Result<String, crate::error::Error> {
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

pub fn parser_register(df: &DataFrame) -> anyhow::Result<DataFrame, crate::error::Error> {
        let filled_df = df
        .clone()
        .lazy()
        .select([col("*").fill_null_with_strategy(FillNullStrategy::Forward(None))])
        .collect()?;

    tracing::debug!("{}", filled_df);

    let parsed_df = filled_df
        .clone()
        .lazy()
        .with_columns(&[
            col("WIDTH")
                .cast(DataType::Int64)
                .sum()
                .over(&[col("ADDR")])
                .alias("REG_WIDTH"),
            (col("WIDTH")
                .cast(DataType::Int64)
                .sum()
                .over(&[col("ADDR")])
                / lit(8))
            .alias("BYTES"),
            coalesce(&[col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"(.*?)\{n\}"), 1)])
            .alias("BASE_REG"),
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .contains(lit(r"\{n\}"), false)
                .alias("IS_EXPANDABLE"),
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"n\s*=\s*(\d+)"), 1)
                .cast(DataType::Int64)
                .alias("START"),
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"~\s*(\d+)"), 1)
                .cast(DataType::Int64)
                .alias("END"),
            col("ADDR")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit("0x([0-9a-fA-F]+)"), 1)
                .str()
                .to_integer(lit(16), false)
                .alias("BASE_ADDR"),
            col("BIT")
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"\[(?:\d+:)?(\d+)]"), 1)
                .alias("BIT_OFFSET")
        ])
        .with_column(
            when(
                col("IS_EXPANDABLE")
                    .and(col("START").is_not_null())
                    .and(col("END").is_not_null()),
            )
            .then(int_ranges(
                col("START"),
                col("END") + lit(1),
                lit(1),
                DataType::Int64,
            ))
            .otherwise(lit(Null {}))
            .alias("N_SERIES"),
        )
        .explode(&["N_SERIES"])
        .filter(
            col("IS_EXPANDABLE")
                .and(col("N_SERIES").is_not_null())
                .or(col("IS_EXPANDABLE")
                    .not()
                    .and(col("FIELD").is_not_null())
                    .and(col("FIELD").neq(lit("")))),
        )
        .with_columns(&[
            when(col("IS_EXPANDABLE"))
                .then(
                    (col("BASE_ADDR").cast(DataType::Int64)
                    + col("N_SERIES").cast(DataType::Int64)
                    * col("BYTES").cast(DataType::Int64))
                        .map(|s| {
                            let ca = s.i64()?;
                            let new_ca: StringChunked = ca
                                .into_iter()
                                .map(|opt_x| opt_x.map(|x| format!("0x{:X}", x)))
                                .collect();
                            Ok(Some(new_ca.into_column()))
                        }, GetOutput::from_type(DataType::String))
                )
                .otherwise(col("ADDR"))
                .alias("ADDR"),
            when(col("IS_EXPANDABLE"))
                .then(
                    col("BASE_REG").cast(DataType::String)
                        + lit("_")
                        + col("N_SERIES").cast(DataType::String),
                )
                .otherwise(col("REG"))
                .alias("REG"),
        ])
        .collect()?;

    tracing::debug!("{}", parsed_df);

    let grouped_df = parsed_df
        .clone()
        .lazy()
        .group_by_stable(["REG"])
        .agg([
            col("ADDR"),
            col("REG_WIDTH"),
            col("FIELD"),
            // col("BIT"),
            col("WIDTH"),
            col("ATTRIBUTE"),
            col("BYTES"),
            col("BIT_OFFSET"),
            col("DEFAULT"),
            // col("BASE_REG"),
            // col("IS_EXPANDABLE"),
            // col("START"),
            // col("END"),
            // col("BASE_ADDR"),
            // col("N_SERIES"),
        ])
        .collect()?;

    tracing::info!("{}", grouped_df);

    // let vec = schema::base::dataframe_to_fields(parsed_df)?;

    // for i in vec.iter() {
    //     tracing::info!("{:#?}", i);
    // }

    Ok(grouped_df)
}

    // let df = df!(
    //     "ADDR" => &[
    //         Some("0x0"), Some("0x4"), None, None, None,
    //         Some("0x8"), Some("0xc"), Some("0x10"), Some("0x20"), None
    //     ],
    //     "REG" => &[
    //         Some("reg0"), Some("reg1"), None, None, None,
    //         Some("reg2"), Some("reg3"), Some("rega{n}, n=0~3"), Some("regb{n}, n=0~3"), None
    //     ],
    //     "FIELD" => &[
    //         Some("field0"), Some("reserved"), Some("field1"), Some("reserved"), Some("field0"),
    //         Some("field0"), Some("field0"), Some("field0"), Some("field1"), Some("field0")
    //     ],
    //     "BIT" => &[
    //         Some("[31:0]"), Some("[31:24]"), Some("[23:16]"), Some("[15:8]"), Some("[7:0]"),
    //         Some("[31:0]"), Some("[31:0]"), Some("[31:0]"), Some("[31:16]"), Some("[15:0]")
    //     ],
    //     "WIDTH" => &[
    //         Some("32"), Some("8"), Some("8"), Some("8"), Some("8"),
    //         Some("32"), Some("32"), Some("32"), Some("16"), Some("16")
    //     ],
    //     "ATTRIBUTE" => &[
    //         Some("RW"), Some("RO"), Some("RW"), Some("RO"), Some("RW"),
    //         Some("W1C"), Some("RC"), Some("RW"), Some("RW"), Some("RW")
    //     ],
    //     "DEFAULT" => &[
    //         Some("0x1234"), Some("0"), Some("0"), Some("0"), Some("0"),
    //         Some("0"), Some("0"), Some("0"), Some("0"), Some("0")
    //     ]
    // )?;