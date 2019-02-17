#include "pm_sensor/server.h"
#include "pm_sensor/credentials.h"


void pm_sensor::Server::start() {
  network_responder.start();
}

void pm_sensor::Server::respond(const uint8_t* request_data, int length) {
  uint8_t buffer[8];

  buffer[0] = (data.current_temperature_humidity.temperature & 0xff00) >> 8;
  buffer[1] = data.current_temperature_humidity.temperature & 0xff;
  buffer[2] = (data.current_temperature_humidity.humidity & 0xff00) >> 8;
  buffer[3] = data.current_temperature_humidity.humidity & 0xff;
  buffer[4] = (data.current_pm.pm2_5 & 0xff00) >> 8;
  buffer[5] = data.current_pm.pm2_5 & 0xff;
  buffer[6] = (data.current_pm.pm10 & 0xff00) >> 8;
  buffer[7] = data.current_pm.pm10 & 0xff;

  network_responder.sendResponse(buffer, 8);
}

void pm_sensor::Server::tick() {
  network_responder.tick();
}
