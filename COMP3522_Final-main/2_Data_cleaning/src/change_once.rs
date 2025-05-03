use itertools::izip;
use polars::prelude::*;
use std::collections::HashSet;
use std::ops::Add;

fn distance((lx, ly, rx, ry): (&f64, &f64, &f64, &f64)) -> f64 {
    let r = 6378.137;

    let dx = lx - rx;
    let dy = ly - ry;

    let a = f64::sin(dy / 2.0).powi(2) + f64::cos(*ly) * f64::cos(*rx) * f64::sin(dx / 2.0).powi(2);
    let c = 2.0 * f64::atan2(f64::sqrt(a), f64::sqrt(1.0 - a));
    return r * c;
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

    let no_change = LazyCsvReader::new("no_change_transport.csv")
        .with_schema(Some(Arc::new(Schema::from_iter(vec![
            Field::new("TRANSPORT".into(), DataType::String),
            Field::new("ROUTE_ID".into(), DataType::String),
            Field::new("ROUTE_SEQ".into(), DataType::String),
            Field::new("STOP_SEQ".into(), DataType::UInt32),
            Field::new("NEXT_STOP_SEQ".into(), DataType::UInt32),
            Field::new("DISTANCE".into(), DataType::Float64),
            Field::new("STOP_X".into(), DataType::Float64),
            Field::new("STOP_Y".into(), DataType::Float64),
            Field::new("STOP_ID".into(), DataType::String),
            Field::new("DISTRICT".into(), DataType::String),
            Field::new("PRICE".into(), DataType::Float64),
            Field::new("NEXT_STOP_X".into(), DataType::Float64),
            Field::new("NEXT_STOP_Y".into(), DataType::Float64),
            Field::new("NEXT_STOP_ID".into(), DataType::String),
            Field::new("NEXT_DISTRICT".into(), DataType::String),
        ]))))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
        .lazy();

    let mut all_routes = vec![];
    for start in districts.iter() {
        let can_go = no_change
            .clone()
            .filter(col("DISTRICT").eq(lit(*start)))
            .collect()
            .unwrap();
        let mut can_go = can_go
            .column("NEXT_DISTRICT")
            .unwrap()
            .str()
            .unwrap()
            .into_iter()
            .map(Option::unwrap)
            .collect::<HashSet<_>>();
        can_go.insert(*start);
        let can_t_go = districts.difference(&can_go);
        for end in can_t_go {
            let routes = no_change
                .clone()
                .filter(col("DISTRICT").eq(lit(*start)))
                .join_builder()
                .with(no_change.clone().filter(col("NEXT_DISTRICT").eq(lit(*end))))
                .how(JoinType::Cross)
                .finish()
                .with_column(
                    as_struct(vec![
                        col("NEXT_STOP_X"),
                        col("NEXT_STOP_Y"),
                        col("STOP_X_right"),
                        col("STOP_Y_right"),
                    ])
                    .map(
                        |s| {
                            let [ref lx, ref ly, ref rx, ref ry] = s
                                .struct_()
                                .unwrap()
                                .fields_as_series()
                                .into_iter()
                                .map(|f| {
                                    f.f64()
                                        .unwrap()
                                        .into_iter()
                                        .map(Option::unwrap)
                                        .collect::<Vec<_>>()
                                })
                                .collect::<Vec<_>>()[..]
                            else {
                                panic!("Shouldn't happen");
                            };
                            return Ok(Some(
                                Series::from_iter(izip!(lx, ly, rx, ry).map(distance))
                                    .into_column(),
                            ));
                        },
                        SpecialEq::default(),
                    )
                    .cast(DataType::Float64)
                    .alias("WALKING_DISTANCE"),
                )
                .filter(col("WALKING_DISTANCE").lt(0.5))
                .with_columns([
                    col("DISTANCE")
                        .add(col("DISTANCE_right"))
                        .add(col("WALKING_DISTANCE"))
                        .alias("SUM_DISTANCE"),
                    col("PRICE").add(col("PRICE_right")).alias("SUM_PRICE"),
                ]);
            all_routes.push(routes);
        }
    }
    concat(&all_routes, UnionArgs::default())
        .unwrap()
        .with_streaming(true)
        .sink_csv(
            "change_transport_once.csv",
            CsvWriterOptions::default(),
            None,
        )
        .unwrap();
}
