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
  respond();
}

void pm_sensor::Server::respond() {
  uint8_t buffer[8];

  buffer[0] = data.current_temperature & 0xff;
  buffer[1] = (data.current_temperature & 0xff00) >> 8;
  buffer[2] = data.current_humidity & 0xff;
  buffer[3] = (data.current_humidity & 0xff00) >> 8;
  buffer[4] = data.current_pm2_5 & 0xff;
  buffer[5] = (data.current_pm2_5 & 0xff00) >> 8;
  buffer[6] = data.current_pm10 & 0xff;
  buffer[7] = (data.current_pm10 & 0xff00) >> 8;

  udp.beginPacket(udp.remoteIP(), udp.remotePort());
  udp.write(buffer, 8);
  udp.endPacket();
}

void pm_sensor::Server::tick() {
  int packet_size = udp.parsePacket();
  if (packet_size) {
    handle(packet_size);
  }
}
