#pragma once

#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

namespace pm_sensor {
  const int INCOMING_BUFFER_SIZE = 256;

  class Server {
  public:
    Server() {};
    void start();
    void tick();

  private:
    void handle(int packet_size);
    void respond();

    WiFiUDP udp;
    uint8_t incoming_buffer[INCOMING_BUFFER_SIZE];
  };
}
