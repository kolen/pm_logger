#pragma once

#include "pm_sensor/data_store.h"
#include "pm_sensor/network_responder.h"

namespace pm_sensor {
  const int INCOMING_BUFFER_SIZE = 256;

  class Server {
  public:
  Server(DataStore& data, NetworkResponder& network_responder) : data(data), network_responder(network_responder)
    {
      network_responder.request_handler = [this] (const uint8_t *request_data, int length) {
	this->respond(request_data, length);
      };
      //network_responder.request_handler = std::bind(&Server::respond, this);
    };
    void start();
    void tick();
  private:
    void respond(const uint8_t* request, int length);

    void respondGetCurrent();
    void respondGetRecorded(const uint8_t* request_data, int length);
    void respondGetRecordedBoundaries(const uint8_t* request_data, int length);

    DataStore& data;
    NetworkResponder& network_responder;
  };
}
