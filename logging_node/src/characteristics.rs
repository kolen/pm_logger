use chrono::{DateTime, Utc};

pub trait Characteristic: Sized {}

macro_rules! value10x {
    ($field:ident) => {
        pub fn $field(&self) -> f32 { self.$field as f32 * 0.1 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TemperatureHumidity {
    pub temperature: i16,
    pub humidity: i16,
}

impl TemperatureHumidity {
    value10x!(temperature);
    value10x!(humidity);
}

impl Characteristic for TemperatureHumidity {}

#[derive(Debug, Clone, Copy)]
pub struct PM {
    pub pm2_5: i16,
    pub pm10: i16,
}

impl PM {
    value10x!(pm2_5);
    value10x!(pm10);
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
