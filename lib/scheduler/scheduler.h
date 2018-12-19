#include <functional>

class Scheduler {
 public:
  void set_hourly(std::function<bool()>);
  void tick();
 private:
  std::function<bool()> scheduled = []() { return true; };
};
