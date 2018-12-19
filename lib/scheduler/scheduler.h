#include <functional>

using scheduler_callback = std::function<bool(const time_t)>;

class Scheduler {
 public:
  void set_hourly(scheduler_callback);
  void tick(const time_t);
 private:
  scheduler_callback scheduled = [](time_t _t) { return true; };
};
