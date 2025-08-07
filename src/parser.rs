use crate::error::Error;
use polars::prelude::*;

pub fn parser_register(df: DataFrame) -> anyhow::Result<DataFrame, Error> {

    // Unmerge cells and distribute content to each cell
    let filled_df = df
        .lazy()
        .select([col("*").fill_null_with_strategy(FillNullStrategy::Forward(None))]);

    let parsed_df = filled_df
        .with_columns(&[
            // TODO
            col("WIDTH")
                .cast(DataType::UInt32)
                .sum()
                .over(&[col("ADDR")])
                .cast(DataType::String)
                .alias("REG_WIDTH"),
            // TODO
            (col("WIDTH")
                .cast(DataType::UInt32)
                .sum()
                .over(&[col("ADDR")])
                / lit(8))
                .alias("BYTES"),
            // TODO
            coalesce(&[col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"(.*?)\{n\}"), 1)])
                .alias("BASE_REG"),
            // TODO
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .contains(lit(r"\{n\}"), false)
                .alias("IS_EXPANDABLE"),
            // TODO
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"n\s*=\s*(\d+)"), 1)
                .cast(DataType::UInt32)
                .alias("START"),
            // TODO
            col("REG")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"~\s*(\d+)"), 1)
                .cast(DataType::UInt32)
                .alias("END"),
            // TODO
            col("ADDR")
                .first()
                .over(&[col("ADDR")])
                .str()
                .extract(lit("0x([0-9a-fA-F]+)"), 1)
                .str()
                .to_integer(lit(16), Some(DataType::UInt32), false)
                .alias("BASE_ADDR"),
            // TODO
            col("BIT")
                .over(&[col("ADDR")])
                .str()
                .extract(lit(r"\[(?:\d+:)?(\d+)]"), 1)
                .alias("BIT_OFFSET"),
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
                DataType::UInt32,
            ))
            .otherwise(lit(Null {}))
            .alias("N_SERIES"),
        )
        .explode(by_name(["N_SERIES"], true))
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
                    (col("BASE_ADDR").cast(DataType::UInt32)
                        + col("N_SERIES").cast(DataType::UInt32)
                            * col("BYTES").cast(DataType::UInt32))
                    .map(
                        |s| {
                            let ca = s.u32()?;
                            let new_ca: StringChunked = ca
                                .into_iter()
                                .map(|opt_x| opt_x.map(|x| format!("0x{:X}", x)))
                                .collect();
                            Ok(Some(new_ca.into_column()))
                        },
                        GetOutput::from_type(DataType::String),
                    ),
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
            when(col("DESCRIPTION").is_null())
                .then(lit("No Description"))
                .otherwise(col("DESCRIPTION"))
                .alias("DESCRIPTION")
        ]);

    let grouped_df = parsed_df
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
            col("DESCRIPTION"),
            // col("BASE_REG"),
            // col("IS_EXPANDABLE"),
            // col("START"),
            // col("END"),
            // col("BASE_ADDR"),
            // col("N_SERIES"),
        ])
        .collect()?;

    tracing::debug!("{}", grouped_df);

    Ok(grouped_df)
}