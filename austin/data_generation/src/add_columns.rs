use itertools::izip;
use polars::prelude::*;

macro_rules! column_iter {
    ($col:expr, $type:ident) => {
        $col.$type().unwrap().into_iter().map(Option::unwrap)
    };
}

fn with_columns_expr(main: LazyFrame, on: [&'static str; 4], add: &[&'static str]) -> Vec<Expr> {
    add.into_iter()
        .map(|field| {
            let field = *field;
            let main = main.clone();
            col(on[0])
                .map_many(
                    move |c| {
                        let c0 = column_iter!(c[0], str);
                        let c1 = column_iter!(c[1], str);
                        let c2 = column_iter!(c[2], str);
                        let c3 = column_iter!(c[3], u32);

                        let i = izip!(c0, c1, c2, c3).map(|(c0, c1, c2, c3)| {
                            main.clone()
                                .filter(
                                    col(on[0])
                                        .eq(lit(c0))
                                        .and(col(on[1]).eq(lit(c1)))
                                        .and(col(on[2]).eq(lit(c2)))
                                        .and(col(on[3]).eq(lit(c3))),
                                )
                                .select([col(field)])
                                .collect()
                                .unwrap()
                                .column(field)
                                .unwrap()
                                .str()
                                .unwrap()
                                .into_iter()
                                .map(Option::unwrap)
                                .nth(0)
                                .unwrap()
                                .to_string()
                        });
                        Ok(Some(Series::from_iter(i).into_column()))
                    },
                    &[col(on[1]), col(on[2]), col(on[3])],
                    SpecialEq::default(),
                )
                .alias(field)
        })
        .collect()
}

pub fn main() {
    let no_change = LazyCsvReader::new("no_change_transport.csv")
        .with_schema(Some(Arc::new(Schema::from_iter(vec![
            Field::new("TRANSPORT".into(), DataType::String),
            Field::new("ROUTE_ID".into(), DataType::String),
            Field::new("ROUTE_SEQ".into(), DataType::String),
            Field::new("STOP_SEQ".into(), DataType::UInt32),
            Field::new("NEXT_STOP_SEQ".into(), DataType::UInt32),
            Field::new("DISTANCE".into(), DataType::Float64),
        ]))))
        .finish()
        .unwrap()
        .with_streaming(true);
    let main = LazyCsvReader::new("main.csv")
        .with_schema(Some(Arc::new(Schema::from_iter(vec![
            Field::new("TRANSPORT".into(), DataType::String),
            Field::new("ROUTE_ID".into(), DataType::String),
            Field::new("COMPANY_CODE".into(), DataType::String),
            Field::new("ROUTE_NAMEE".into(), DataType::String),
            Field::new("CIRCULAR".into(), DataType::Boolean),
            Field::new("ROUTE_SEQ".into(), DataType::String),
            Field::new("STOP_SEQ".into(), DataType::UInt32),
            Field::new("STOP_ID".into(), DataType::String),
            Field::new("STOP_X".into(), DataType::String),
            Field::new("STOP_Y".into(), DataType::String),
            Field::new("STOP_PICK_DROP".into(), DataType::String),
            Field::new("STOP_NAME".into(), DataType::String),
            Field::new("DISTRICT".into(), DataType::String),
            Field::new("NEXT_STOP_SEQ".into(), DataType::UInt32),
            Field::new("NEXT_STOP_ID".into(), DataType::String),
            Field::new("NEXT_STOP_X".into(), DataType::String),
            Field::new("NEXT_STOP_Y".into(), DataType::String),
            Field::new("NEXT_STOP_NAME".into(), DataType::String),
            Field::new("NEXT_DISTRICT".into(), DataType::String),
            Field::new("DISTANCE".into(), DataType::Float64),
            Field::new("PRICE".into(), DataType::String),
        ]))))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
        .lazy();

    let no_change = no_change
        .with_columns(&with_columns_expr(
            main.clone(),
            ["TRANSPORT", "ROUTE_ID", "ROUTE_SEQ", "STOP_SEQ"],
            &["STOP_X", "STOP_Y", "STOP_ID", "DISTRICT", "PRICE"],
        ))
        .with_columns(&with_columns_expr(
            main.clone(),
            ["TRANSPORT", "ROUTE_ID", "ROUTE_SEQ", "NEXT_STOP_SEQ"],
            &[
                "NEXT_STOP_X",
                "NEXT_STOP_Y",
                "NEXT_STOP_ID",
                "NEXT_DISTRICT",
            ],
        ));
    let no_change = concat(
        &[
            no_change,
            main.filter(col("TRANSPORT").eq(lit("MTR"))).select([
                col("TRANSPORT"),
                col("ROUTE_ID"),
                col("ROUTE_SEQ"),
                col("STOP_SEQ"),
                col("NEXT_STOP_SEQ"),
                col("DISTANCE"),
                col("STOP_X"),
                col("STOP_Y"),
                col("STOP_ID"),
                col("DISTRICT"),
                col("PRICE"),
                col("NEXT_STOP_X"),
                col("NEXT_STOP_Y"),
                col("NEXT_STOP_ID"),
                col("NEXT_DISTRICT"),
            ]),
        ],
        UnionArgs::default(),
    )
    .unwrap();
    let mut out = no_change.collect().unwrap();

    let f = std::fs::File::create("no_change_transport.csv").unwrap();
    CsvWriter::new(f).finish(&mut out).unwrap();
}
