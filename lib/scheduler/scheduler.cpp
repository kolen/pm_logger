#include "scheduler.h"

void Scheduler::set_hourly(std::function<bool(time_t time)> scheduled) {
  this->scheduled = scheduled;
}

void Scheduler::tick(time_t time) {
  scheduled(time);
}
