#include "scheduler.h"

void Scheduler::set_hourly(std::function<bool()> scheduled) {
  this->scheduled = scheduled;
}

void Scheduler::tick() {
  scheduled();
}
