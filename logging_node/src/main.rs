extern crate env_logger;
extern crate logging_node;

use dotenv::dotenv;
use logging_node::characteristics::{TemperatureHumidity, PM};
use logging_node::database;
use std::rc::Rc;
use logging_node::puller::Puller;

fn main() {
    dotenv().ok();
    env_logger::init();

    let conn = Rc::new(database::establish_connection());
    let puller = Puller::new(conn);

    puller.update_characteristic::<PM>().unwrap();
    puller.update_characteristic::<TemperatureHumidity>().unwrap();
}
