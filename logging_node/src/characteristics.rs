use crate::schema::measurements_pm;
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

pub trait CharacteristicFromColumns {
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

// measurements_temp_humidity table has the same column structure, and
// therefore the same SqlType, as measurements_pm, so that works for
// it too
impl<C> Queryable<measurements_pm::SqlType, Sqlite> for Sample<C>
where
    C: CharacteristicFromColumns + Characteristic,
{
    type Row = (i32, Option<i32>, Option<i32>);

    fn build(row: Self::Row) -> Self {
        Sample {
            time: DateTime::from_utc(NaiveDateTime::from_timestamp(row.0 as i64, 0), Utc),
            value: row.1.and_then(|c1| {
                row.2.and_then(|c2| {
                    Some(<C as CharacteristicFromColumns>::characteristic_from_columns(c1, c2))
                })
            }),
        }
    }
}

// It's very hard to implement Insertable for that, so no Insertable
// for now. Also you can't just return impl Insertable in
// non-polymorphic function, you have to specify its Value type, which
// is hard. Probably going to wait for existential types
// https://rust-lang.github.io/rfcs/2071-impl-trait-existential-types.html
// and their support in Diesel
//
// See also: https://stackoverflow.com/a/49096462/123642

// Making something coposable for Diesel is currently hard due to
// types issues

/// Makes expression for `.values` to insert `Sample<pm>` into database.
///
/// ```no_run
/// # #[macro_use] extern crate logging_node;
/// use diesel::insert_into;
/// use diesel::sqlite::SqliteConnection;
/// use diesel::prelude::*;
/// use chrono::prelude::*;
/// use logging_node::characteristics::{Sample, PM};
///
/// let my_sample = Sample {
///     time: Utc::now(),
///     value: Some(PM { pm2_5: 100, pm10: 200 }),
/// };
/// let connection = SqliteConnection::establish(":memory:")
///     .unwrap();
///
/// insert_into(logging_node::schema::measurements_pm::table)
///     .values(insert_pm_values!(my_sample))
///     .execute(&connection)
///     .unwrap();
/// ```
///
#[macro_export]
macro_rules! insert_pm_values {
    ($sample:expr) => {
        (
            $crate::schema::measurements_pm::time.eq(($sample).time.timestamp() as i32),
            $crate::schema::measurements_pm::pm2_5.eq(($sample).value.map(|vv| vv.pm2_5 as i32)),
            $crate::schema::measurements_pm::pm10.eq(($sample).value.map(|vv| vv.pm10 as i32)),
        )
    };
}

/// See insert_pm_values
#[macro_export]
macro_rules! insert_temp_humidity_values {
    ($sample:expr) => {
        (
            $crate::schema::measurements_temp_humidity::time.eq(($sample).time.timestamp() as i32),
            $crate::schema::measurements_temp_humidity::temperature
                .eq(($sample).value.map(|vv| vv.temperature as i32)),
            $crate::schema::measurements_temp_humidity::humidity
                .eq(($sample).value.map(|vv| vv.humidity as i32)),
        )
    };
}
