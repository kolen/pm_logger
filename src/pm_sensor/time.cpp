#include "pm_sensor/time.h"
#include "pm_sensor/logging.h"
#ifdef ARDUINO
#else
 #include <time.h>
#endif

using pm_sensor::Time;
using pm_sensor::Logging;

#ifdef ARDUINO
const int NTP_PACKET_SIZE = 48;
const int NTP_LOCAL_PORT = 8123;

uint8_t packetBuffer[NTP_PACKET_SIZE];
WiFiUDP Udp;

static const char ntp_server[] PROGMEM = "time.google.com";

static void sendNTPpacket(const char* address)
{
  memset(packetBuffer, 0, NTP_PACKET_SIZE);
  packetBuffer[0] = 0b11100011; // LI, Version, Mode
  packetBuffer[1] = 0;          // Stratum, or type of clock
  packetBuffer[2] = 6;          // Polling Interval
  packetBuffer[3] = 0xEC;       // Peer Clock Precision
  // 8 bytes of zero for Root Delay & Root Dispersion
  packetBuffer[12]  = 49;
  packetBuffer[13]  = 0x4E;
  packetBuffer[14]  = 49;
  packetBuffer[15]  = 52;

  Udp.beginPacket(address, 123);
  Udp.write(packetBuffer, NTP_PACKET_SIZE);
  Udp.endPacket();
}

static time_t getNtpTime()
{
  while (Udp.parsePacket() > 0) ; // discard any previously received packets
  Logging::println(F("Transmit NTP Request"));
  sendNTPpacket(ntp_server);
  uint32_t beginWait = millis();
  while (millis() - beginWait < 1500) {
    int size = Udp.parsePacket();
    if (size >= NTP_PACKET_SIZE) {
      Logging::println(F("Receive NTP Response"));
      Udp.read(packetBuffer, NTP_PACKET_SIZE);
      unsigned long secsSince1900;
      secsSince1900 =  (unsigned long)packetBuffer[40] << 24;
      secsSince1900 |= (unsigned long)packetBuffer[41] << 16;
      secsSince1900 |= (unsigned long)packetBuffer[42] << 8;
      secsSince1900 |= (unsigned long)packetBuffer[43];
      return secsSince1900 - 2208988800UL * SECS_PER_HOUR;
    }
  }
  Logging::println(F("No NTP Response"));
  return 0;
}
#endif

void Time::tick() {
}

void Time::start() {
  #ifdef ARDUINO
  Udp.begin(NTP_LOCAL_PORT);
  setSyncProvider(getNtpTime);
  #endif
}

int32_t Time::now() {
  #ifdef ARDUINO
  return (int32_t)now();
  #else
  const auto base = 1551727673;
  return (int32_t)((time(NULL)-base)*60 + base);
  #endif
}
