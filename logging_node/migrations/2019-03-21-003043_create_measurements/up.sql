create table measurements_pm (
  time integer primary key not null,
  pm2_5 integer,
  pm10 integer
);

create table measurements_temp_humidity (
  time integer primary key not null,
  temperature integer,
  humidity integer
);
