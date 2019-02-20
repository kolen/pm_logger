#pragma once
#ifndef ARDUINO
#include "pm_sensor/data_store.h"
#include "pm_sensor/sensor_pm.h"
#include <iostream>

namespace pm_sensor {
  class SensorPMDeviceFake : public SensorPMDevice {
  public:
    void start();
    void setSleepMode(bool sleep);
    PMMeasurement measure();
  };
}

#endif
