#include "pm_sensor/time.h"
#include "pm_sensor/logging.h"
#ifdef ARDUINO
#else
 #include <time.h>
#endif

using pm_sensor::Time;
using pm_sensor::Logging;

static const int32_t SYNC_INTERVAL = 60 * 60 * 24 * 5;

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

static int32_t getNTPTime()
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

#else

static int32_t getNTPTime() {
  // No real need to use NTP for simulation
  return (int32_t)time(NULL);
}

#endif

void Time::tick() {
  // calling code calls now() every tick anyways, so it's currently
  // empty
}

void Time::start() {
  syncIfNecessary();
  #ifdef ARDUINO
  Udp.begin(NTP_LOCAL_PORT);
  #endif
}

#ifdef ARDUINO

static inline unsigned long millis_impl() {
  return millis();
}

#else

static inline unsigned long millis_impl() {
  // const auto base = 1551727673;
  // return (int32_t)((time(NULL)-base)*60 + base);

  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);

  uint64_t millis = ts.tv_sec * 1000;
  millis += ts.tv_nsec / 1000000;
  return (int32_t)millis * 60;
}

#endif

void Time::syncIfNecessary() {
  if (time_since_last_sync < 0 || time_since_last_sync > SYNC_INTERVAL) {
    Logging::println(PSTR("Syncing time"));
    int32_t synced_time = getNTPTime();
    if (!synced_time) { return; }

    Logging::print(PSTR("Synced time: "));
    Logging::println(synced_time);
    last_millis = millis_impl();
    current_time = synced_time;
    time_since_last_sync = 0;
  }
}

int32_t Time::now() {
  //printf("Current millis: %ld, time: %d, since_last_sync: %d\n", millis_impl(), current_time, time_since_last_sync);
  syncIfNecessary();

  // Time is meaningful only if time base is set
  if (!current_time) { return 0; }
  unsigned long current_millis = millis_impl();
  unsigned long interval_from_last = current_millis - last_millis;
  int32_t seconds_from_last = (int32_t)(interval_from_last / 1000);
  current_time += seconds_from_last;
  time_since_last_sync += seconds_from_last;

  last_millis = (current_millis / 1000) * 1000;

  return current_time;
}
