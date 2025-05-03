use itertools::izip;
use polars::prelude::*;

pub fn main() {
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
        .select([
            col("TRANSPORT"),
            as_struct(vec![col("STOP_SEQ"), col("NEXT_STOP_SEQ")])
                .map(stop_diff_2, SpecialEq::default())
                .cast(DataType::UInt32)
                .alias("NO_STOPS"),
            col("DISTANCE"),
            col("PRICE"),
            col("DISTRICT"),
            col("NEXT_DISTRICT"),
        ]);

    let change_once = LazyCsvReader::new("change_transport_once.csv")
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
            Field::new("TRANSPORT_right".into(), DataType::String),
            Field::new("ROUTE_ID_right".into(), DataType::String),
            Field::new("ROUTE_SEQ_right".into(), DataType::String),
            Field::new("STOP_SEQ_right".into(), DataType::UInt32),
            Field::new("NEXT_STOP_SEQ_right".into(), DataType::UInt32),
            Field::new("DISTANCE_right".into(), DataType::Float64),
            Field::new("STOP_X_right".into(), DataType::Float64),
            Field::new("STOP_Y_right".into(), DataType::Float64),
            Field::new("STOP_ID_right".into(), DataType::String),
            Field::new("DISTRICT_right".into(), DataType::String),
            Field::new("PRICE_right".into(), DataType::Float64),
            Field::new("NEXT_STOP_X_right".into(), DataType::Float64),
            Field::new("NEXT_STOP_Y_right".into(), DataType::Float64),
            Field::new("NEXT_STOP_ID_right".into(), DataType::String),
            Field::new("NEXT_DISTRICT_right".into(), DataType::String),
            Field::new("WALKING_DISTANCE".into(), DataType::Float64),
            Field::new("SUM_DISTANCE".into(), DataType::Float64),
            Field::new("SUM_PRICE".into(), DataType::Float64),
        ]))))
        .finish()
        .unwrap()
        .collect()
        .unwrap()
        .lazy()
        .select([
            as_struct(vec![col("TRANSPORT"), col("TRANSPORT_right")])
                .map(a_to_b, SpecialEq::default())
                .cast(DataType::String)
                .alias("TRANSPORT"),
            as_struct(vec![
                col("STOP_SEQ"),
                col("NEXT_STOP_SEQ"),
                col("STOP_SEQ_right"),
                col("NEXT_STOP_SEQ_right"),
            ])
            .map(stop_diff_4, SpecialEq::default())
            .cast(DataType::UInt32)
            .alias("NO_STOPS"),
            col("SUM_DISTANCE").alias("DISTANCE"),
            col("SUM_PRICE").alias("PRICE"),
            col("DISTRICT"),
            col("NEXT_DISTRICT_right").alias("NEXT_DISTRICT"),
        ]);

    let mut a = concat(vec![no_change, change_once], UnionArgs::default())
        .unwrap()
        .collect()
        .unwrap();

    let f = std::fs::File::create("combined.csv").unwrap();
    CsvWriter::new(f).finish(&mut a).unwrap();
}

fn a_to_b(s: Column) -> PolarsResult<Option<Column>> {
    let [ref l, ref r] = s
        .struct_()
        .unwrap()
        .fields_as_series()
        .into_iter()
        .map(|s| {
            s.str()
                .unwrap()
                .into_iter()
                .map(Option::unwrap)
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()[..]
    else {
        panic!("Shouldn't happen");
    };
    return Ok(Some(
        Series::from_iter(izip!(l, r).map(|(l, r)| format!("{}->{}", l, r))).into_column(),
    ));
}

fn stop_diff_2(s: Column) -> PolarsResult<Option<Column>> {
    let [ref a, ref b] = s
        .struct_()
        .unwrap()
        .fields_as_series()
        .into_iter()
        .map(|s| {
            s.u32()
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
        Series::from_iter(izip!(a, b).map(|(a, b)| b - a)).into_column(),
    ));
}

fn stop_diff_4(s: Column) -> PolarsResult<Option<Column>> {
    let [ref a, ref b, ref c, ref d] = s
        .struct_()
        .unwrap()
        .fields_as_series()
        .into_iter()
        .map(|s| {
            s.u32()
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
        Series::from_iter(izip!(a, b, c, d).map(|(a, b, c, d)| b - a + d - c)).into_column(),
    ));
}
