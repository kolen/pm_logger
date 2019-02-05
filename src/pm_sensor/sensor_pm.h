#pragma once

#include <functional>
#include "pm_sensor/data_store.h"

namespace pm_sensor {
  enum class SensorPMState { idle, warmup, measure, failed_measure };

  class SensorPMDevice {
  public:
    virtual ~SensorPMDevice() { };
    virtual void start() = 0;
    virtual void setSleepMode(bool sleep) = 0;
    virtual pm_sensor::PMMeasurement measure() = 0;
  };

  class SensorPM {
  public:
  SensorPM(std::function<void(PMMeasurement)> callback, SensorPMDevice& device):
    callback(callback), device(device) { };
    void start();
    void tick(int32_t time);
    void measure();
  private:
    std::function<void(PMMeasurement)> callback;
    SensorPMDevice& device;
    SensorPMState state = SensorPMState::idle;
    int32_t measure_time = 0;
    int32_t failed_measure_time = 0;
  };
}
