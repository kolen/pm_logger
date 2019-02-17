#include "pm_sensor/network_responder.h"
#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>

namespace pm_sensor {
  const int MAX_QUERY_SIZE = 16; // TODO: tweak this

  class ArduinoNetworkResponder: public NetworkResponder {
  public:
    virtual void start();
    virtual void tick();
    virtual void sendResponse(const uint8_t* buffer, int length);
  private:
    WiFiUDP udp;
    uint8_t query_buffer[MAX_QUERY_SIZE];
  };
}
