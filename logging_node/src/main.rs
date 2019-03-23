extern crate env_logger;
#[macro_use]
extern crate logging_node;

use chrono::Duration;
use diesel::insert_or_ignore_into;
use diesel::prelude::*;
use logging_node::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use logging_node::database;
use logging_node::puller;
use logging_node::schema;

fn all_samples<C>(puller: &puller::Puller) -> Vec<(Sample<C>)>
where
    C: Characteristic + puller::NetworkedCharacteristic,
{
    // TODO: get rid of unwraps
    let characteristic = <C as puller::NetworkedCharacteristic>::query_characteristic();
    let boundaries = puller.get_boundaries(characteristic).unwrap();

    let interval = match characteristic {
        puller::QueryCharacteristic::PM => Duration::hours(1),
        puller::QueryCharacteristic::TemperatureHumidity => Duration::minutes(10),
    };

    (0..boundaries.num_samples)
        .scan(boundaries.last_sample_at, |time, _i| {
            let current_time = *time;
            let value = puller.get_recorded(current_time).unwrap();
            *time = *time - interval;
            Some(Sample {
                time: current_time,
                value: value,
            })
        })
        .collect()
}

fn main() {
    env_logger::init();

    let conn = database::establish_connection();

    let puller = puller::Puller::new("pm_sensor.local:12000").unwrap();

    println!(
        "{:?} {:?}",
        puller.get_boundaries(puller::QueryCharacteristic::PM),
        puller.get_boundaries(puller::QueryCharacteristic::TemperatureHumidity)
    );

    println!("All PM samples:");
    let all_pm = all_samples::<PM>(&puller);
    println!("{:?}", all_pm);

    for sample in &all_pm {
        insert_or_ignore_into(schema::measurements_pm::table)
            .values(insert_pm_values!(&sample))
            .execute(&conn)
            .unwrap();
    }

    // println!(
    //     "{:?}",
    //     measurements_pm::table.limit(1).load::<Sample<PM>>(&conn)
    // );
    // println!(
    //     "{:?}",
    //     measurements_temp_humidity::table
    //         .limit(1)
    //         .load::<Sample<PM>>(&conn)
    // );

    let all_temp = all_samples::<TemperatureHumidity>(&puller);

    for sample in &all_temp {
        insert_or_ignore_into(schema::measurements_temp_humidity::table)
            .values(insert_temp_humidity_values!(&sample))
            .execute(&conn)
            .unwrap();
    }

    println!("All temperature samples:");
    println!("{:?}", all_temp);
}
