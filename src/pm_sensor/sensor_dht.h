#pragma once

#include <functional>
#include "pm_sensor/data_store.h"

#ifdef ARDUINO
 #include <DHT.h>
#endif

namespace pm_sensor {
  class SensorDHT {
  public:
  SensorDHT(std::function<void(TemperatureHumidityMeasurement)> callback, int dht_pin):
    #ifdef ARDUINO
    callback(callback), dht(dht_pin, DHT22), retry_at(0)
    #else
    callback(callback), retry_at(0)
    #endif
      { };

    void start();
    void tick(int32_t time);
    void measure();
  private:
    std::function<void(TemperatureHumidityMeasurement)> callback;
    #ifdef ARDUINO
    DHT dht;
    #endif
    int32_t retry_at;
  };
}
