use chrono::{DateTime, Utc};

pub trait Characteristic: Sized {}

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
