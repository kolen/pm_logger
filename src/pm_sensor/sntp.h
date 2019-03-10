#pragma once

#include <cstdint>
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

namespace pm_sensor {
  class SNTPClient {
  public:
    SNTPClient(const char* host, int local_port): host(host), local_port(local_port) {};
    int32_t query();
    void start();
  private:
    WiFiUDP udp;
    const char* host;
    int local_port;
  };
}
