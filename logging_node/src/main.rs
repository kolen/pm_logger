extern crate env_logger;
extern crate logging_node;

use dotenv::dotenv;
use logging_node::characteristics::{Pressure, TemperatureHumidity, PM};
use logging_node::database;
use logging_node::puller::Puller;
use std::rc::Rc;

fn main() {
    dotenv().ok();
    env_logger::init();

    let conn = Rc::new(database::establish_connection());
    let puller = Puller::new(conn);

    puller.update_characteristic::<PM>().unwrap();
    puller
        .update_characteristic::<TemperatureHumidity>()
        .unwrap();
    puller.update_characteristic::<Pressure>().unwrap();
}
