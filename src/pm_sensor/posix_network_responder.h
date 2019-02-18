#pragma once
#include "pm_sensor/network_responder.h"

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <stdio.h>
#include <unistd.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>

namespace pm_sensor {
  const int MAX_QUERY_SIZE = 16; // TODO: tweak this

  class PosixNetworkResponder: public NetworkResponder {
  public:
    virtual void start();
    virtual void tick();
    virtual void sendResponse(const uint8_t* buffer, int length);
  private:
    int sockfd;
    struct sockaddr_in server_addr;
    struct sockaddr_in client_addr;
  };
}
