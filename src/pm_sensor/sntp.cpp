#include "pm_sensor/sntp.h"
#include "pm_sensor/logging.h"

using pm_sensor::SNTPClient;

const int ntp_port = 123;
const int timeout_millis = 2000;

void SNTPClient::start() {
  udp.begin(local_port);
}

int32_t SNTPClient::query() {
  Logging::println(FLS("Querying SNTP"));
  while(udp.parsePacket() > 0);

  uint8_t packet[48];
  memset(packet, 0, 48);
  packet[0] = 0b11100011;

  udp.beginPacket(host, ntp_port);
  udp.write(packet, 48);
  udp.endPacket();

  auto poll_start = millis();
  while(millis() - poll_start < timeout_millis) {
    if (udp.parsePacket() >= 48) {
      udp.read(packet, 48);
      return
	((int32_t)packet[40] << 24 |
	(int32_t)packet[41] << 16 |
	(int32_t)packet[42] << 8 |
	(int32_t)packet[43]) - 2208988800;
    }
    delay(100);
  }
  Logging::println(FLS("SNTP response timeout"));
  delay(10000); // TODO: handle retries properly
  return 0;
}
