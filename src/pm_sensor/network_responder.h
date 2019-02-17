#pragma once
#include <functional>

const int NETWORK_MAXIMUM_REQUEST_SIZE = 8;

namespace pm_sensor {
  class NetworkResponder {
  public:
    virtual ~NetworkResponder() { };
    virtual void start() { };
    virtual void tick() { };
    virtual void sendResponse(const uint8_t* buffer, int length) = 0;

    std::function<void (const uint8_t* request_data, int length)> request_handler {};
  };
}
