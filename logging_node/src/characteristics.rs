pub trait Characteristic {}

pub struct TemperatureHumidity {
    pub temperature: i16,
    pub humidity: i16,
}

impl Characteristic for TemperatureHumidity {}

pub struct PM {
    pub pm2_5: i16,
    pub pm10: i16,
}


impl Characteristic for PM {}
