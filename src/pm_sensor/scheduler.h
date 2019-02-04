#pragma once

#include <cstdint>
#include <functional>

namespace pm_sensor {
  class Scheduler {
  public:
    std::function<void (int32_t)> hourly_callback {};
    std::function<void (int32_t)> minutely_callback {};
    uint32_t hourly_hours_mask = 0;
    int minutely_period = 0;

    void tick(int32_t current_time);
  private:
    int32_t last_known_time = 0;
    int32_t hourly_last_run = 0;
    int32_t minutely_last_run = 0;
  };
}
