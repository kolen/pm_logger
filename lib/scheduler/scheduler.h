#include <functional>

using scheduler_callback = std::function<bool(const time_t)>;

bool should_run_at(time_t now, time_t last_successful_run, bool last_is_failure);

class Scheduler {
 public:
  void set_hourly(scheduler_callback);
  void tick(const time_t);
 private:
  scheduler_callback scheduled = [](time_t _t) { return true; };
  time_t last_successful_run = 0;
  bool last_is_failure = false;
};
