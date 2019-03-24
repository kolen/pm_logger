use crate::characteristics::{Characteristic, Sample, TemperatureHumidity, PM};
use crate::client::{Client, ClientError, NetworkedCharacteristic};
use crate::schema::measurements_pm;
use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::{insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};
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

// Dunno how to do polymorphism on Disel level, i.e. substitute types
// for table, for time column, etc. So lots of duplication here

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

impl StorableCharacteristic for PM {
    fn retrieve_dates_for_range(
        range: RangeInclusive<DateTime<Utc>>,
        connection: &SqliteConnection,
    ) -> Result<HashSet<DateTime<Utc>>, diesel::result::Error> {
        Ok(measurements_pm::table
            .select(measurements_pm::time)
            .filter(measurements_pm::time.between(
                range.start().timestamp() as i32,
                range.end().timestamp() as i32,
            ))
            .load::<i32>(connection)?
            .into_iter()
            .map(|ts| Utc.timestamp(ts as i64, 0))
            .collect())
    }

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
    /// device but isn't currently stored in the database
    pub fn update_characteristic<C>(&self) -> Result<(), PullerError>
    where
        C: Characteristic + NetworkedCharacteristic + StorableCharacteristic,
    {
        debug!("Update characteristic");
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
        let unfilled_times = &all_times - &database_times;
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
}
