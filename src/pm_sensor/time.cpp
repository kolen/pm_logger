#include "pm_sensor/time.h"
#include "pm_sensor/logging.h"
#ifdef ARDUINO
 #include "pm_sensor/sntp.h"
#else
 #include <time.h>
#endif

using pm_sensor::Time;
using pm_sensor::Logging;
#ifdef ARDUINO
using pm_sensor::SNTPClient;
#endif

static const int32_t SYNC_INTERVAL = 60 * 60 * 24 * 5;

#ifdef ARDUINO

SNTPClient sntp_client("time.google.com", 12000);

static int32_t getNTPTime() {
  return sntp_client.query();
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
  #ifdef ARDUINO
  sntp_client.start();
  #endif
  syncIfNecessary();
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
    Logging::println(FLS("Syncing time"));
    int32_t synced_time = getNTPTime();
    if (!synced_time) { return; }

    Logging::print(FLS("Synced time: "));
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
