#ifndef ARDUINO
#include "pm_sensor/sensor_pm_fake.h"
#include "pm_sensor/data_store.h"

using pm_sensor::SensorPMDeviceFake;
using pm_sensor::PMMeasurement;

void SensorPMDeviceFake::start() {
  std::cout << "SDS011 device start" << std::endl;
}

void SensorPMDeviceFake::setSleepMode(bool sleep) {
  std::cout << "Set sleep mode: " << sleep << std::endl;
}

PMMeasurement SensorPMDeviceFake::measure() {
  return PMMeasurement { 123, 456 };
}

#endif
