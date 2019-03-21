table! {
    measurements_pm (time) {
        time -> Integer,
        pm2_5 -> Nullable<Integer>,
        pm10 -> Nullable<Integer>,
    }
}

table! {
    measurements_temp_humidity (time) {
        time -> Integer,
        temperature -> Nullable<Integer>,
        humidity -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    measurements_pm,
    measurements_temp_humidity,
);
