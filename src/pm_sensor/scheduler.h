#pragma once

#include <cstdint>
#include <functional>

namespace pm_sensor {
  class HourlyScheduler {
  public:
    std::function<void (int32_t)> callback {};
    uint32_t hours_mask = 0;

    void tick(int32_t current_time);
  private:
    int32_t last_known_time = 0;
    int32_t last_run = 0;
  };

  class MinutelyScheduler {
  public:
    std::function<void (int32_t)> callback {};
    int period = 0;

    void tick(int32_t current_time);
  private:
    int32_t last_known_time = 0;
    int32_t last_run = 0;
  };
}
