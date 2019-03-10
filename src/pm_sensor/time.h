#pragma once

#ifdef ARDUINO
 #include <TimeLib.h>
 #include <ESP8266WiFi.h>
 #include <WiFiUdp.h>
#else
 #include <cstdint>
#endif

namespace pm_sensor {
  class Time {
  public:
    void start();
    void tick();
    int32_t now();
  private:
    int32_t current_time = 0;
    int32_t time_since_last_sync = -1;
    unsigned long last_millis = 0;
    void syncIfNecessary();
  };
}
