#pragma once
#include <SdsDustSensor.h>
#include "pm_sensor/data_store.h"
#include "pm_sensor/sensor_pm.h"

namespace pm_sensor {
  class SensorPMDeviceSDS011 : public SensorPMDevice {
  public:
  SensorPMDeviceSDS011(int rx_pin, int tx_pin): sds(rx_pin, tx_pin) { };
    void start();
    void setSleepMode(bool sleep);
    PMMeasurement measure();
  private:
    SdsDustSensor sds;
  };
}
