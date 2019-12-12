#include "pm_sensor/data_store.h"
#include <cmath>

int16_t pm_sensor::float_to_stored(float value) {
  return (int16_t) round(value * 10.0);
}

float pm_sensor::stored_to_float(int16_t value) {
  return value / 10.0;
}

#ifndef ARDUINO
namespace pm_sensor {
  std::ostream &operator<<(std::ostream &os, TemperatureHumidityMeasurement const &m) {
    os << "temp=" << m.temperature << ", humidity=" << m.humidity;
    return os;
  }

  std::ostream &operator<<(std::ostream &os, pm_sensor::PMMeasurement const &m) {
    os << "PM2.5=" << m.pm2_5 << ", PM10=" << m.pm10;
    return os;
  }
}
#endif
