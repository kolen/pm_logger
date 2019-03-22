use crate::schema::{measurements_pm, measurements_temp_humidity};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::sqlite::Sqlite;
use diesel::Queryable;

pub trait Characteristic {}

#[derive(Debug, Clone, Copy)]
pub struct TemperatureHumidity {
    pub temperature: i16,
    pub humidity: i16,
}

impl Characteristic for TemperatureHumidity {}

#[derive(Debug, Clone, Copy)]
pub struct PM {
    pub pm2_5: i16,
    pub pm10: i16,
}

impl Characteristic for PM {}

#[derive(Debug, Clone, Copy)]
pub struct Sample<C>
where
    C: Characteristic,
{
    pub time: DateTime<Utc>,
    pub value: Option<C>,
}

trait CharacteristicFromColumns {
    fn characteristic_from_columns(c1: i32, c2: i32) -> Self;
}

impl CharacteristicFromColumns for PM {
    fn characteristic_from_columns(c1: i32, c2: i32) -> Self {
        PM {
            pm2_5: c1 as i16,
            pm10: c2 as i16,
        }
    }
}

impl CharacteristicFromColumns for TemperatureHumidity {
    fn characteristic_from_columns(c1: i32, c2: i32) -> Self {
        TemperatureHumidity {
            temperature: c1 as i16,
            humidity: c2 as i16,
        }
    }
}
