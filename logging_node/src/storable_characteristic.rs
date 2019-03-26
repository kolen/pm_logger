use crate::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use crate::schema::{measurements_pm, measurements_temp_humidity};
use chrono::{DateTime, TimeZone, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub trait StorableCharacteristic: Characteristic {
    fn retrieve_dates_for_range(
        range: RangeInclusive<DateTime<Utc>>,
        connection: &SqliteConnection,
    ) -> Result<HashSet<DateTime<Utc>>, diesel::result::Error>;
    fn insert_sample(
        sample: Sample<Self>,
        connection: &SqliteConnection,
    ) -> Result<(), diesel::result::Error>;
}

// Dunno how to do polymorphism on Diesel level (what are correct
// types for tables, columns, etc), so here's C++ templates approach,
// haha

macro_rules! retrieve_dates_for_range_impl {
    ($table:ident) => {
        fn retrieve_dates_for_range(
            range: RangeInclusive<DateTime<Utc>>,
            connection: &SqliteConnection,
        ) -> Result<HashSet<DateTime<Utc>>, diesel::result::Error> {
            Ok($table::table
                .select($table::time)
                .filter($table::time.between(
                    range.start().timestamp() as i32,
                    range.end().timestamp() as i32,
                ))
                .load::<i32>(connection)?
                .into_iter()
                .map(|ts| Utc.timestamp(ts as i64, 0))
                .collect())
        }
    };
}

impl StorableCharacteristic for PM {
    retrieve_dates_for_range_impl!(measurements_pm);

    fn insert_sample(
        sample: Sample<Self>,
        connection: &SqliteConnection,
    ) -> Result<(), diesel::result::Error> {
        insert_into(measurements_pm::table)
            .values(insert_pm_values!(&sample))
            .execute(connection)
            .map(|_| ())
    }
}

impl StorableCharacteristic for TemperatureHumidity {
    retrieve_dates_for_range_impl!(measurements_temp_humidity);

    fn insert_sample(
        sample: Sample<Self>,
        connection: &SqliteConnection,
    ) -> Result<(), diesel::result::Error> {
        insert_into(measurements_temp_humidity::table)
            .values(insert_temp_humidity_values!(&sample))
            .execute(connection)
            .map(|_| ())
    }
}
