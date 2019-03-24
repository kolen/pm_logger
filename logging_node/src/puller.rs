use crate::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use crate::client::{Client, ClientError, NetworkedCharacteristic};
use crate::schema::{measurements_pm, measurements_temp_humidity};
use chrono::{DateTime, TimeZone, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::collections::HashSet;
use std::env;
use std::ops::RangeInclusive;
use std::rc::Rc;

pub struct Puller {
    client: Client,
    connection: Rc<SqliteConnection>,
}

#[derive(Debug, Fail)]
pub enum PullerError {
    #[fail(display = "timeout waiting for response from device")]
    Timeout(#[fail(cause)] ClientError),
    #[fail(display = "sql error: {}", 0)]
    SqlError(#[fail(cause)] diesel::result::Error),
}

impl From<ClientError> for PullerError {
    fn from(e: ClientError) -> Self {
        PullerError::Timeout(e)
    }
}

impl From<diesel::result::Error> for PullerError {
    fn from(e: diesel::result::Error) -> Self {
        PullerError::SqlError(e)
    }
}

// Some storage-related functionality is in Characteristic module by
// itself, so it's good idea to consolidate it into single place

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

impl Puller {
    pub fn new(connection: Rc<SqliteConnection>) -> Self {
        Puller {
            client: Client::new(
                env::var("LOGGER_HOST").expect("LOGGER_HOST env variable must be set"),
            )
            .unwrap(),
            connection: connection,
        }
    }

    /// Update samples of characteristic that are available on the
    /// device but isn't currently stored in the database. Polymorphic
    /// to characteristic type.
    pub fn update_characteristic<C>(&self) -> Result<(), PullerError>
    where
        C: Characteristic + NetworkedCharacteristic + StorableCharacteristic,
    {
        debug!(
            "Update characteristic {}",
            <C as NetworkedCharacteristic>::query_characteristic()
        );
        let unfilled_times = self.unfilled_times::<C>()?;
        debug!("Requesting new samples: {:?}", unfilled_times);

        for time in unfilled_times {
            let result: Result<Option<C>, _> = self.client.get_recorded(time);
            match result {
                Ok(value) => {
                    let sample = Sample {
                        time: time,
                        value: value,
                    };
                    <C as StorableCharacteristic>::insert_sample(sample, &*self.connection)?;
                }
                Err(e) => {
                    warn!("Error receiving sample @{}: {:?}", time, e);
                    continue;
                }
            }
        }
        Ok(())
    }

    /// Returns set of points in time which does not have samples
    /// stored in database
    fn unfilled_times<C>(&self) -> Result<HashSet<DateTime<Utc>>, PullerError>
    where
        C: Characteristic + NetworkedCharacteristic + StorableCharacteristic,
    {
        let boundaries = self
            .client
            .get_boundaries(<C as NetworkedCharacteristic>::query_characteristic())?;
        debug!("Retured boundaries: {}", boundaries);
        let database_times: HashSet<DateTime<Utc>> =
            <C as StorableCharacteristic>::retrieve_dates_for_range(
                boundaries.date_range(),
                &*self.connection,
            )?;
        let all_times: HashSet<DateTime<Utc>> = boundaries.times().collect();
        Ok(&all_times - &database_times)
    }
}
