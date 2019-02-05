#pragma once
#include <cstdint>
#include "pm_sensor/data_recorder.h"

namespace pm_sensor {
  const int temp_humidity_period = 10 * 60;
  const int temp_humidity_capacity = 24 * 10;

  int16_t float_to_stored(float value);

  struct TemperatureHumidityMeasurement {
  public:
  TemperatureHumidityMeasurement(): temperature(0), humidity(0) {};
  TemperatureHumidityMeasurement(float temperature, float humidity):
    temperature(float_to_stored(temperature)),
      humidity(float_to_stored(humidity)) { }
    int16_t temperature;
    int16_t humidity;
  };

  struct PMMeasurement {
  public:
  PMMeasurement(): pm2_5(0), pm10(0) {};
  PMMeasurement(float pm2_5, float pm10):
    pm2_5(float_to_stored(pm2_5)), pm10(float_to_stored(pm10)) { }
    int16_t pm2_5;
    int16_t pm10;
    operator bool() const { return pm2_5 && pm10; };
  };

  class DataStore {
  public:
  DataStore() :
    current_temperature_humidity(),
      current_pm(),
      temp_humidity_recorder(temp_humidity_data, temp_humidity_capacity, temp_humidity_period)
	{ }

    TemperatureHumidityMeasurement current_temperature_humidity;
    PMMeasurement current_pm;

    TemperatureHumidityMeasurement temp_humidity_data[temp_humidity_capacity];
    DataRecorder<TemperatureHumidityMeasurement> temp_humidity_recorder;
  };
}
