extern crate env_logger;
#[macro_use]
extern crate logging_node;

use chrono::Duration;
use diesel::insert_or_ignore_into;
use diesel::prelude::*;
use dotenv::dotenv;
use logging_node::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use logging_node::client;
use logging_node::database;
use logging_node::schema;

fn all_samples<C>(puller: &client::Client) -> Vec<(Sample<C>)>
where
    C: Characteristic + client::NetworkedCharacteristic,
{
    // TODO: get rid of unwraps
    let characteristic = <C as client::NetworkedCharacteristic>::query_characteristic();
    let boundaries = puller.get_boundaries(characteristic).unwrap();

    let interval = match characteristic {
        client::QueryCharacteristic::PM => Duration::hours(1),
        client::QueryCharacteristic::TemperatureHumidity => Duration::minutes(10),
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
    dotenv().ok();
    env_logger::init();

    let conn = database::establish_connection();

    let client = client::Client::new("pm_sensor.local:12000").unwrap();

    println!(
        "{:?} {:?}",
        client.get_boundaries(client::QueryCharacteristic::PM),
        client.get_boundaries(client::QueryCharacteristic::TemperatureHumidity)
    );

    println!("All PM samples:");
    let all_pm = all_samples::<PM>(&client);
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

    let all_temp = all_samples::<TemperatureHumidity>(&client);

    for sample in &all_temp {
        insert_or_ignore_into(schema::measurements_temp_humidity::table)
            .values(insert_temp_humidity_values!(&sample))
            .execute(&conn)
            .unwrap();
    }

    println!("All temperature samples:");
    println!("{:?}", all_temp);
}
