extern crate env_logger;
extern crate logging_node;

use diesel::prelude::*;
use logging_node::characteristics::{Characteristic, TemperatureHumidity, PM};
use logging_node::database;
use logging_node::models::*;
use logging_node::puller;
use logging_node::schema;
use std::time;

fn all_samples<C>(puller: &puller::Puller) -> Vec<(time::SystemTime, C)>
where
    C: Characteristic + puller::NetworkedCharacteristic,
{
    // TODO: get rid of unwraps
    let characteristic = <C as puller::NetworkedCharacteristic>::query_characteristic();
    let boundaries = puller.get_boundaries(characteristic).unwrap();

    let interval = time::Duration::from_secs(match characteristic {
        puller::QueryCharacteristic::PM => 60 * 60,
        puller::QueryCharacteristic::TemperatureHumidity => 60 * 10,
    });

    (0..boundaries.num_samples)
        .scan(boundaries.last_sample_at, |time, _i| {
            let current_time = *time;
            let value = puller.get_recorded(current_time).unwrap();
            *time = *time - interval;
            Some((current_time, value))
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

    for (time, value_pm) in all_pm {
        let record = NewPM {
            time: time.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i32,
            pm2_5: value_pm.pm2_5 as i32,
            pm10: value_pm.pm10 as i32,
        };

        diesel::insert_into(schema::measurements_pm::table)
            .values(&record)
            .execute(&conn)
            .unwrap();
    }

    let all_temp = all_samples::<TemperatureHumidity>(&puller);

    for (time, value_pm) in &all_temp {
        let record = NewTempHumidity {
            time: time.duration_since(time::UNIX_EPOCH).unwrap().as_secs() as i32,
            temperature: value_pm.temperature as i32,
            humidity: value_pm.humidity as i32,
        };

        diesel::insert_into(schema::measurements_temp_humidity::table)
            .values(&record)
            .execute(&conn)
            .unwrap();
    }

    println!("All temperature samples:");
    println!("{:?}", all_temp);
}
