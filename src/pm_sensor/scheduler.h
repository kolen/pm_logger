#pragma once

#include <cstdint>
#include <functional>

namespace pm_sensor {
  class Scheduler {
  public:
  Scheduler(): hourly_callback(), minutely_callback(), hourly_hours_mask(0), minutely_period(0),
      last_known_time(0), hourly_last_run(0), minutely_last_run(0) { };
    std::function<void (int32_t)> hourly_callback;
    std::function<void (int32_t)> minutely_callback;
    uint32_t hourly_hours_mask;
    int minutely_period;

    void tick(int32_t current_time);
  private:
    int32_t last_known_time;
    int32_t hourly_last_run;
    int32_t minutely_last_run;
  };
}
