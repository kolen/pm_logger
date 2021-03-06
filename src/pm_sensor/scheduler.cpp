#include "pm_sensor/scheduler.h"
#include <cstring>

#define SECONDS_IN_HOUR (60*60)

static int32_t scheduler_beginning_of_hour(int32_t time) {
  return time - (time % SECONDS_IN_HOUR);
}

static int scheduler_hour_number(int32_t time) {
  return (time % (SECONDS_IN_HOUR * 24)) / SECONDS_IN_HOUR;
}

static int scheduler_should_run_in_hour(int hour_number, uint32_t hours_mask) {
  return hours_mask & (1 << (23 - hour_number));
}

void pm_sensor::HourlyScheduler::tick(int32_t current_time) {
  if (!(last_known_time < current_time)) {
    return; // Detected "backwards clock", might happen when correcting it
  }
  last_known_time = current_time;

  int32_t current_hour = scheduler_beginning_of_hour(current_time);
  int current_hour_number = scheduler_hour_number(current_hour);

  if (current_hour > last_run &&
      scheduler_should_run_in_hour(current_hour_number, hours_mask)) {
    last_run = current_hour;
    callback(current_hour);
  }
}

static int32_t scheduler_beginning_of_minute(int32_t time) {
  return time - (time % 60);
}

static int scheduler_minute_number(int32_t time) {
  return (time % SECONDS_IN_HOUR) / 60;
}

static int scheduler_should_run_in_minute(int minute_number, int period) {
  return period && ((minute_number % period) == 0);
}

void pm_sensor::MinutelyScheduler::tick(int32_t current_time) {
  if (!(last_known_time < current_time)) {
    return; // Detected "backwards clock", might happen when correcting it
  }
  last_known_time = current_time;

  int32_t current_minute = scheduler_beginning_of_minute(current_time);
  int current_minute_number = scheduler_minute_number(current_minute);

  if (current_minute > last_run &&
      scheduler_should_run_in_minute(current_minute_number, period)) {
    last_run = current_minute;
    callback(current_minute);
  }
}
