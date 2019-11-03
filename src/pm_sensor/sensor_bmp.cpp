#include "pm_sensor/sensor_bmp.h"
#include "pm_sensor/logging.h"

using pm_sensor::SensorBMP;

void SensorBMP::start() {
  #ifdef ARDUINO
  auto success = bmp.begin();
  if (!success) {
    Logging::print(FLS("Can't initialize BMP"));
  }
  #endif
}

void SensorBMP::measure() {
  int32_t pressure;
  #ifdef ARDUINO
  pressure = bmp.readPressure();
  #else
  pressure = 100000;
  #endif

  Logging::print(FLS("Pressure: "));
  Logging::print(pressure);

  callback(pressure);
}

void SensorBMP::tick(int32_t time) {
}
