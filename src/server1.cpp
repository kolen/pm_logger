#include "server1.h"
#include "credentials.h"

const unsigned int udp_port = 12000;

void pm_sensor::Server::start() {
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);

  Serial.println("Network server starting");
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
  }
  Serial.println("Connected to wifi");

  udp.begin(udp_port);
}

void pm_sensor::Server::handle(int packet_size) {
  int len = udp.read(incoming_buffer, INCOMING_BUFFER_SIZE);
}

void pm_sensor::Server::respond() {
  udp.beginPacket(udp.remoteIP(), udp.remotePort());
  udp.write("Test");
  udp.endPacket();
}

void pm_sensor::Server::tick() {
  int packet_size = udp.parsePacket();
}
