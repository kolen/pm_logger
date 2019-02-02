#include <cstdint>

class DataStore {
 public:
  DataStore() {};
  int32_t current_temperature = 0;
  int32_t current_humidity = 0;
  int32_t current_pm2_5 = 0;
  int32_t current_pm10 = 0;
};
