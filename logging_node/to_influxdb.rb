require 'bundler/inline'

gemfile do
  source 'https://rubygems.org'
  gem 'influxdb', '~> 0.8.0'
  gem 'sqlite3', '1.4.2'
end

influxdb = InfluxDB::Client.new(
  'sensors',
  host: "bresso.local",
  time_precision: 's'
)

db = SQLite3::Database.new "#{Dir.home}/.logging_node/logging_node.sqlite"

last_time = 1601251200

data = db.execute(
  "select time, pressure from measurements_pressure where time > ?", last_time
).map do |ts, pressure|
  {
    series: "pressure",
    values: { value: pressure.to_i },
    timestamp: ts
  }
end

influxdb.write_points data

data = db.execute(
  "select time, pm2_5, pm10 from measurements_pm where time > ?", last_time
).flat_map do |ts, pm2_5, pm10|
  [
    {
      series: "pm2_5",
      values: { value: pm2_5.to_i },
      timestamp: ts
    },
    {
      series: "pm10",
      values: { value: pm10.to_i },
      timestamp: ts
    },
  ]
end

influxdb.write_points data

data = db.execute(
  "select time, temperature, humidity from measurements_temp_humidity where time > ?", last_time
).flat_map do |ts, temperature, humidity|
  [
    {
      series: "temperature",
      values: { value: temperature.to_i },
      timestamp: ts
    },
    {
      series: "humidity",
      values: { value: humidity.to_i },
      timestamp: ts
    },
  ]
end

influxdb.write_points data
