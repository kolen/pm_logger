#pragma once

#include <functional>

#ifdef ARDUINO
 #include <Adafruit_BMP085.h>
#endif

namespace pm_sensor {
  class SensorBMP {
  public:
    SensorBMP(std::function<void(int32_t)> callback): callback(callback) { };
    void start();
    void tick(int32_t time);
    void measure();
  private:
    std::function<void(int32_t)> callback;
  };
}
