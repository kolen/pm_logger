#pragma once
#include <SdsDustSensor.h>
#include "pm_sensor/data_store.h"
#include "pm_sensor/sensor_pm.h"

namespace pm_sensor {
  class SensorPMDeviceSDS011 : public SensorPMDevice {
  public:
    SensorPMDeviceSDS011(int rx_pin, int tx_pin, int switch_pin):
      sds(rx_pin, tx_pin), switch_pin(switch_pin) { };
    void start();
    void setSleepMode(bool sleep);
    PMMeasurement measure();
    void idleCheck();
  private:
    SdsDustSensor sds;
    int switch_pin;
  };
}
