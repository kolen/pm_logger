#pragma once
#include <cstdint>

namespace pm_sensor {
  class DataStore {
  public:
    DataStore() {};
    int16_t current_temperature = 0;
    int16_t current_humidity = 0;
    int16_t current_pm2_5 = 0;
    int16_t current_pm10 = 0;

    static int16_t float_to_stored(float value);
  };
}
