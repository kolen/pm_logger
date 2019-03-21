extern crate logging_node;
extern crate env_logger;

use logging_node::puller;
use logging_node::characteristics::{Characteristic, TemperatureHumidity, PM};
use std::time;

fn all_samples<C>(puller: &puller::Puller) -> Vec<(time::SystemTime, C)>
where C: Characteristic + puller::NetworkedCharacteristic {
    // TODO: get rid of unwraps
    let characteristic = <C as puller::NetworkedCharacteristic>::query_characteristic();
    let boundaries = puller.get_boundaries(characteristic).unwrap();

    let interval = time::Duration::from_secs(match characteristic {
        puller::QueryCharacteristic::PM => 60 * 60,
        puller::QueryCharacteristic::TemperatureHumidity => 60 * 10,
    });

    (0..boundaries.num_samples).scan(boundaries.last_sample_at, |time, _i| {
        let current_time = *time;
        let value = puller.get_recorded(current_time).unwrap();
        *time = *time - interval;
        Some((current_time, value))
    }).collect()
}

fn main() {
    env_logger::init();
    let puller = puller::Puller::new("pm_sensor.local:12000").unwrap();

    println!(
        "{:?} {:?}",
        puller.get_boundaries(puller::QueryCharacteristic::PM),
        puller.get_boundaries(puller::QueryCharacteristic::TemperatureHumidity)
    );

    println!("All PM samples:");
    println!("{:?}", all_samples::<PM>(&puller));
    println!("All temperature samples:");
    println!("{:?}", all_samples::<TemperatureHumidity>(&puller));
}
