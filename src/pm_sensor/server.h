#pragma once

#include "pm_sensor/data_store.h"
#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

namespace pm_sensor {
  const int INCOMING_BUFFER_SIZE = 256;

  class Server {
  public:
  Server(DataStore& data) : data(data) {};
    void start();
    void tick();

  private:
    void handle(int packet_size);
    void respond();

    DataStore& data;
    WiFiUDP udp;
    uint8_t incoming_buffer[INCOMING_BUFFER_SIZE];
  };
}
