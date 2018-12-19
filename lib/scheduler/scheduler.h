#include <functional>

class Scheduler {
 public:
  void set_hourly(std::function<bool(const time_t)>);
  void tick(const time_t);
 private:
  std::function<bool(time_t)> scheduled = [](time_t _t) { return true; };
};
