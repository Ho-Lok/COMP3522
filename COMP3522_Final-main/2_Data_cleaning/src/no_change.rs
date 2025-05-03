use itertools::izip;
use polars::prelude::*;
use std::collections::HashSet;

macro_rules! column_iter {
    ($col:expr, $type:ident) => {
        $col.$type().unwrap().into_iter().map(Option::unwrap)
    };
}

pub fn main() {
    let districts = LazyCsvReader::new("districts.csv")
        .finish()
        .unwrap()
        .collect()
        .unwrap();
    let districts = districts.columns(["DISTRICT"]).unwrap()[0]
        .str()
        .unwrap()
        .into_iter()
        .map(Option::unwrap)
        .collect::<HashSet<_>>();

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
            Field::new("STOP_X".into(), DataType::Float64),
            Field::new("STOP_Y".into(), DataType::Float64),
            Field::new("STOP_PICK_DROP".into(), DataType::String),
            Field::new("STOP_NAME".into(), DataType::String),
            Field::new("DISTRICT".into(), DataType::String),
            Field::new("NEXT_STOP_SEQ".into(), DataType::UInt32),
            Field::new("NEXT_STOP_ID".into(), DataType::String),
            Field::new("NEXT_STOP_X".into(), DataType::Float64),
            Field::new("NEXT_STOP_Y".into(), DataType::Float64),
            Field::new("NEXT_STOP_NAME".into(), DataType::String),
            Field::new("NEXT_DISTRICT".into(), DataType::String),
            Field::new("DISTANCE".into(), DataType::Float64),
            Field::new("PRICE".into(), DataType::Float32),
        ]))))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
        .lazy();

    let mut all_routes = vec![];
    for start in districts.iter() {
        for end in districts.iter() {
            let m = main.clone().filter(col("TRANSPORT").neq(lit("MTR")));
            let routes = m
                .clone()
                .filter(
                    col("DISTRICT")
                        .eq(lit(*start))
                        .or(col("NEXT_DISTRICT").eq(lit(*end)))
                        .over(["TRANSPORT", "ROUTE_ID", "ROUTE_SEQ"]),
                )
                .group_by_stable(["TRANSPORT", "ROUTE_ID", "ROUTE_SEQ"])
                .agg([
                    cols(["STOP_SEQ", "DISTRICT"]).filter(col("DISTRICT").eq(lit(*start))),
                    cols(["NEXT_STOP_SEQ", "NEXT_DISTRICT"])
                        .filter(col("NEXT_DISTRICT").eq(lit(*end))),
                ])
                .drop(["DISTRICT", "NEXT_DISTRICT"])
                .explode(["STOP_SEQ"])
                .explode(["NEXT_STOP_SEQ"])
                .filter(col("STOP_SEQ").lt(col("NEXT_STOP_SEQ")))
                .with_column(
                    col("TRANSPORT")
                        .map_many(
                            move |c| {
                                let transport = column_iter!(c[0], str);
                                let route_id = column_iter!(c[1], str);
                                let route_seq = column_iter!(c[2], str);
                                let stop_seq = column_iter!(c[3], u32);
                                let next_stop_seq = column_iter!(c[4], u32);

                                let i =
                                    izip!(transport, route_id, route_seq, stop_seq, next_stop_seq)
                                        .map(|(t, ri, rs, ss, nss)| {
                                            m.clone()
                                                .filter(
                                                    col("TRANSPORT")
                                                        .eq(lit(t))
                                                        .and(col("ROUTE_ID").eq(lit(ri)))
                                                        .and(col("ROUTE_SEQ").eq(lit(rs)))
                                                        .and(col("STOP_SEQ").gt_eq(ss))
                                                        .and(col("NEXT_STOP_SEQ").lt_eq(nss)),
                                                )
                                                .select([col("DISTANCE")])
                                                .sum()
                                                .collect()
                                                .unwrap()
                                                .column("DISTANCE")
                                                .unwrap()
                                                .f64()
                                                .unwrap()
                                                .into_iter()
                                                .nth(0)
                                                .unwrap()
                                        });
                                Ok(Some(Series::from_iter(i).into_column()))
                            },
                            &[
                                col("ROUTE_ID"),
                                col("ROUTE_SEQ"),
                                col("STOP_SEQ"),
                                col("NEXT_STOP_SEQ"),
                            ],
                            SpecialEq::default(),
                        )
                        .cast(DataType::Float64)
                        .alias("DISTANCE"),
                );
            all_routes.push(routes);
        }
    }
    let full = concat(&all_routes, UnionArgs::default()).unwrap();
    let mut out = full.collect().unwrap();

    let f = std::fs::File::create("no_change_transport.csv").unwrap();
    CsvWriter::new(f).finish(&mut out).unwrap();
}
