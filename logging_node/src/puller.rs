use crate::characteristics::{Characteristic, Sample};
use crate::client::{Client, ClientError, NetworkedCharacteristic};
use crate::storable_characteristic::StorableCharacteristic;
use chrono::{DateTime, Utc};
use diesel::sqlite::SqliteConnection;
use std::collections::HashSet;
use std::env;
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

pub enum UpdateResult {
    UpdateComplete,
    UpdateIncomplete,
}

impl Puller {
    pub fn new(connection: Rc<SqliteConnection>) -> Self {
        Puller {
            client: Client::new(
                env::var("LOGGER_HOST").expect("LOGGER_HOST env variable must be set"),
            )
            .unwrap(),
            connection,
        }
    }

    /// Update samples of characteristic that are available on the
    /// device but isn't currently stored in the database. Polymorphic
    /// to characteristic type.
    pub fn update_characteristic<C>(&self) -> Result<UpdateResult, PullerError>
    where
        C: Characteristic + NetworkedCharacteristic + StorableCharacteristic,
    {
        // Multiple logging_node processes could be running in
        // parallel so we wrap whole update process in exclusive
        // transaction
        self.connection
            .exclusive_transaction(|| self.update_characteristic_inner::<C>())
    }

    /// Real meat of update_characteristic that happens inside transaction
    fn update_characteristic_inner<C>(&self) -> Result<UpdateResult, PullerError>
    where
        C: Characteristic + NetworkedCharacteristic + StorableCharacteristic,
    {
        info!(
            "Update characteristic {}",
            <C as NetworkedCharacteristic>::query_characteristic()
        );
        let unfilled_times = self.unfilled_times::<C>()?;
        info!("Requesting new samples: {:?}", unfilled_times);

        let mut complete = true;
        for time in unfilled_times {
            let result: Result<Option<C>, _> = self.client.get_recorded(time);
            match result {
                Ok(value) => {
                    let sample = Sample { time, value };
                    <C as StorableCharacteristic>::insert_sample(sample, &*self.connection)?;
                }
                Err(e) => {
                    warn!("Error receiving sample @{}: {:?}", time, e);
                    complete = false;
                    continue;
                }
            }
        }
        if complete {
            Ok(UpdateResult::UpdateComplete)
        } else {
            Ok(UpdateResult::UpdateIncomplete)
        }
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
        let database_times: HashSet<DateTime<Utc>> = match boundaries.date_range() {
            Some(date_range) => <C as StorableCharacteristic>::retrieve_dates_for_range(
                date_range,
                &*self.connection,
            )?,
            None => HashSet::new(),
        };
        let all_times: HashSet<DateTime<Utc>> = boundaries.times().collect();
        Ok(&all_times - &database_times)
    }
}
