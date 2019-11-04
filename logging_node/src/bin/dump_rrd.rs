extern crate env_logger;
extern crate log;
extern crate logging_node;

use diesel::prelude::*;
use dotenv::dotenv;
use logging_node::characteristics::{Characteristic, Sample, TemperatureHumidity};
use logging_node::database;
use logging_node::schema::measurements_temp_humidity;
use std::process::Command;

fn rrd_format_sample_number<C>(sample: Sample<C>, extract: impl Fn(C) -> f32) -> String
where
    C: Characteristic,
{
    sample
        .value
        .map(extract)
        .map_or("N".into(), |v: f32| v.to_string())
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let connection = database::establish_connection();
    let samples: Vec<Sample<TemperatureHumidity>> = measurements_temp_humidity::table
        .order(measurements_temp_humidity::time.asc())
        .load(&connection)
        .expect("Can't load temp/humidity samples");

    let rrd_keys = samples
        .into_iter()
        .map(|sample: Sample<TemperatureHumidity>| {
            format!(
                "{}:{}:{}",
                sample.time.timestamp(),
                rrd_format_sample_number(sample, |v| v.temperature()),
                rrd_format_sample_number(sample, |v| v.humidity())
            )
        });

    Command::new("rrdtool")
        .args(
            vec![
                String::from("update"),
                String::from("temp_humidity.rrd"),
                String::from("--"),
            ]
            .into_iter()
            .chain(rrd_keys),
        )
        .spawn()
        .unwrap();
}
