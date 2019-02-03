#pragma once
#include <cstdint>
#include "pm_sensor/data_recorder.h"

namespace pm_sensor {
  const int temp_humidity_period = 10 * 60;
  const int temp_humidity_capacity = 24 * 10;

  class TemperatureHumidityMeasurement {
  public:
    int16_t temperature;
    int16_t humidity;
  };

  class DataStore {
  public:
  DataStore() :
    current_temperature(0),
      current_humidity(0),
      current_pm2_5(0),
      current_pm10(0),
      temp_humidity_recorder(temp_humidity_data, temp_humidity_capacity, temp_humidity_period)
	{ }

    int16_t current_temperature;
    int16_t current_humidity;
    int16_t current_pm2_5;
    int16_t current_pm10;

    static int16_t float_to_stored(float value);

    TemperatureHumidityMeasurement temp_humidity_data[temp_humidity_capacity];
    DataRecorder<TemperatureHumidityMeasurement> temp_humidity_recorder;
  };
}
