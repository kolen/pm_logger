#include "pm_sensor/arduino_network_responder.h"
#include <Arduino.h>
#include <ESP8266WiFi.h>
#include <WiFiUdp.h>
#include <ESP8266mDNS.h>

#include "credentials.h"

using pm_sensor::ArduinoNetworkResponder;

const unsigned int udp_port = 12000;

// TODO: separate to its own class
static void mdnsStart() {
  if (!MDNS.begin("pm_sensor")) {
    Serial.println("Error setting up MDNS responder!");
  }

  MDNS.addService("pm_sensor", "udp", udp_port);
}

// TODO: separate to its own class
static void mdnsTick() {
  MDNS.update();
}

void ArduinoNetworkResponder::start() {
  WiFi.hostname("pm-sensor");
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  Serial.println("Network server starting");
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
  }
  Serial.println("Connected to wifi");

  mdnsStart();

  udp.begin(udp_port);
}

void ArduinoNetworkResponder::tick() {
  mdnsTick();
  yield();
  int packet_size;
  do {
    packet_size = udp.parsePacket();
    if (packet_size) {
      int len = udp.read(query_buffer, MAX_QUERY_SIZE);
      if (len) {
	request_handler(query_buffer, len);
      }
    }
    yield();
  } while (packet_size);
}

void ArduinoNetworkResponder::sendResponse(const uint8_t* buffer, int length) {
  udp.beginPacket(udp.remoteIP(), udp.remotePort());
  udp.write(buffer, length);
  udp.endPacket();
}
