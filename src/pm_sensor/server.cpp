#include "pm_sensor/server.h"
#include "pm_sensor/credentials.h"

using pm_sensor::Server;

enum struct RequestType: uint8_t { get_current = 1, get_recorded = 2, get_recorded_boundaries = 3 };
enum struct DataType: uint8_t { pm = 1, temperature = 2 };
enum struct ResponseType: uint8_t { current = 1, recorded = 2, recorded_boundaries = 3 };

void Server::start() {
  network_responder.start();
}

void Server::respondGetCurrent() {
  uint8_t buffer[9];

  buffer[0] = static_cast<uint8_t>(ResponseType::current);
  buffer[1] = (data.current_temperature_humidity.temperature & 0xff00) >> 8;
  buffer[2] = data.current_temperature_humidity.temperature & 0xff;
  buffer[3] = (data.current_temperature_humidity.humidity & 0xff00) >> 8;
  buffer[4] = data.current_temperature_humidity.humidity & 0xff;
  buffer[5] = (data.current_pm.pm2_5 & 0xff00) >> 8;
  buffer[6] = data.current_pm.pm2_5 & 0xff;
  buffer[7] = (data.current_pm.pm10 & 0xff00) >> 8;
  buffer[8] = data.current_pm.pm10 & 0xff;

  network_responder.sendResponse(buffer, 8);
}

void Server::respondGetRecorded(const uint8_t* request_data, int length) {
  // 0: query type (1 byte)
  // 1: date (4 bytes)
  // 5: data type (1 byte)

  // 0: 'recorded' - ResponseType::recorded (1 byte)
  // 1: copy of request (5 bytes)
  // 6: measurement - pm2_5 / temperature (2 bytes)
  // 8: measurement - pm10 / humidity (2 bytes)
  uint8_t response[10];

  if (length != 6) return;

  int32_t time =
    request_data[1] << 24 |
    request_data[2] << 16 |
    request_data[3] << 8 |
    request_data[4];

  response[0] = static_cast<uint8_t>(ResponseType::recorded);
  // Copy time and type from request
  memcpy(response+1, request_data+1, 5);

  switch(static_cast<DataType>(request_data[5])) {
  case DataType::pm:
    {
      auto data_pm = data.pm_recorder.get_at_time(time);
      response[6] = (data_pm.pm2_5 & 0xff00) >> 8;
      response[7] = data_pm.pm2_5 & 0xff;
      response[6] = (data_pm.pm10 & 0xff00) >> 8;
      response[7] = data_pm.pm10 & 0xff;
      network_responder.sendResponse(response, 10);
    }
    break;
  case DataType::temperature:
    {
      auto data_temp = data.temp_humidity_recorder.get_at_time(time);
      response[6] = (data_temp.temperature & 0xff00) >> 8;
      response[7] = data_temp.temperature & 0xff;
      response[6] = (data_temp.humidity & 0xff00) >> 8;
      response[7] = data_temp.humidity & 0xff;
      network_responder.sendResponse(response, 10);
    }
    break;
  }
}

void Server::respondGetRecordedBoundaries(const uint8_t* request_data, int length) {
  // 0: request type
  // 1: data type

  int32_t last_sample_time;
  size_t num_samples;

  if (length != 2) return;

  switch(static_cast<DataType>(request_data[1])) {
  case DataType::pm:
    last_sample_time = data.pm_recorder.last_sample_time;
    num_samples = data.pm_recorder.num_samples_filled;
    break;
  case DataType::temperature:
    last_sample_time = data.temp_humidity_recorder.last_sample_time;
    num_samples = data.temp_humidity_recorder.num_samples_filled;
    break;
  default:
    return;
  }

  uint8_t response[6];
  response[0] = static_cast<uint8_t>(ResponseType::recorded_boundaries);
  response[1] = request_data[1];

  response[2] = (last_sample_time & 0xff000000) >> 24;
  response[3] = (last_sample_time & 0xff0000) >> 16;
  response[4] = (last_sample_time & 0xff00) >> 8;
  response[5] = last_sample_time & 0xff;

  network_responder.sendResponse(response, 6);
}

void Server::respond(const uint8_t* request_data, int length) {
  RequestType request_type = (RequestType)request_data[0];

  switch(request_type) {
  case RequestType::get_current:
    respondGetCurrent();
    break;
  case RequestType::get_recorded:
    respondGetRecorded(request_data, length);
    break;
  case RequestType::get_recorded_boundaries:
    respondGetRecordedBoundaries(request_data, length);
    break;
  }
}

void Server::tick() {
  network_responder.tick();
}
