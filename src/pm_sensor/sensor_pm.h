#pragma once

#include <functional>
#include "pm_sensor/data_store.h"

namespace pm_sensor {
  class SensorPM {
  public:
  SensorPM(std::function<void(PMMeasurement)> callback): callback(callback) { };
    void start();
    void tick(int32_t time);
  private:
    std::function<void(PMMeasurement)> callback;
  };
}
