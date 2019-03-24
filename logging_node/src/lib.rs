#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate failure;

pub mod characteristics;
pub mod client;
pub mod database;
pub mod schema;
