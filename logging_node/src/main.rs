extern crate env_logger;
extern crate logging_node;

use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use logging_node::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use logging_node::database;
use logging_node::puller;
use logging_node::schema;
use logging_node::schema::{measurements_pm, measurements_temp_humidity};

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

    // for (time, value_pm) in all_pm {
    //     let record = NewPM {
    //         time: time.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i32,
    //         pm2_5: value_pm.map(|x| x.pm2_5 as i32),
    //         pm10: value_pm.map(|x| x.pm10 as i32),
    //     };

    println!(
        "{:?}",
        measurements_pm::table.limit(1).load::<Sample<PM>>(&conn)
    );
    println!(
        "{:?}",
        measurements_temp_humidity::table
            .limit(1)
            .load::<Sample<PM>>(&conn)
    );

    //     diesel::insert_into(schema::measurements_pm::table)
    //         .values(&record)
    //         .execute(&conn)
    //         .unwrap();
    // }

    let all_temp = all_samples::<TemperatureHumidity>(&puller);

    // for (time, value_pm) in &all_temp {
    //     let record = NewTempHumidity {
    //         time: time.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i32,
    //         temperature: value_pm.map(|x| x.temperature as i32),
    //         humidity: value_pm.map(|x| x.humidity as i32),
    //     };

    //     diesel::insert_into(schema::measurements_temp_humidity::table)
    //         .values(&record)
    //         .execute(&conn)
    //         .unwrap();
    // }

    println!("All temperature samples:");
    println!("{:?}", all_temp);
}
