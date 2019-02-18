#include "pm_sensor/posix_network_responder.h"
#include <fcntl.h>
#include <iostream>

using pm_sensor::PosixNetworkResponder;

void PosixNetworkResponder::start() {
  std::cout << "Initializing network" << std::endl;
  if ((sockfd = socket(AF_INET, SOCK_DGRAM, 0)) == -1) {
    perror("Can't initialize socket");
    exit(1);
  }
  if (fcntl(sockfd, F_SETFL, O_NONBLOCK) == -1) {
    perror("Can't make socket non-blocking");
    exit(1);
  }

  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(12000);
  server_addr.sin_addr.s_addr = INADDR_ANY;
  bzero(&(server_addr.sin_zero), 8);

  if (bind(sockfd, (struct sockaddr *)&server_addr,
	   sizeof(struct sockaddr)) == -1) {
    perror("Can't bind socket");
    exit(1);
  }
}

// Have to do polling, like original Arduino code
void PosixNetworkResponder::tick() {
  uint8_t recv_data[256];
  socklen_t addr_size = sizeof client_addr;
  int bytes_read = recvfrom(sockfd, recv_data, 256, 0,
			    (struct sockaddr *)&client_addr, &addr_size);
  if (bytes_read == -1) {
    if (errno == EAGAIN || errno == EWOULDBLOCK) {
      return;
    } else {
      perror("Can't read from socket");
      exit(1);
    }
  }
  request_handler(recv_data, bytes_read);
}

void PosixNetworkResponder::sendResponse(const uint8_t* buffer, int length) {
  socklen_t addr_size = sizeof client_addr;
  int result = sendto(sockfd, buffer, length, 0,
		      (struct sockaddr *)&client_addr, addr_size);
  if (result == -1) {
    perror("Can't send to socket");
    exit(1);
  }
}
