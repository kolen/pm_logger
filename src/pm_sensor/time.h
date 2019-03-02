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
    static void start();
    static void tick();
    static int32_t now();
  };
}
