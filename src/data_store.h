#pragma once
#include <cstdint>

namespace pm_sensor {
  class DataStore {
  public:
    DataStore() {};
    int32_t current_temperature = 0;
    int32_t current_humidity = 0;
    int32_t current_pm2_5 = 0;
    int32_t current_pm10 = 0;

    static int32_t float_to_stored(float value);
  };
}
