use super::schema::*;

#[derive(Queryable)]
pub struct PM {
    pub time: i64,
    pub pm2_5: i16,
    pub pm10: i16,
}

#[derive(Queryable)]
pub struct TempHumidity {
    pub time: i64,
    pub temperature: i16,
    pub humidity: i16,
}

#[derive(Insertable)]
#[table_name="measurements_pm"]
pub struct NewPM {
    pub time: i32, // TODO: try to map to SystemTime
    pub pm2_5: i32,
    pub pm10: i32,
}

#[derive(Insertable)]
#[table_name="measurements_temp_humidity"]
pub struct NewTempHumidity {
    pub time: i32, // TODO: try to map to SystemTime
    pub temperature: i32,
    pub humidity: i32,
}
