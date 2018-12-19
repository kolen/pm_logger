#include "scheduler.h"

void Scheduler::set_hourly(scheduler_callback scheduled) {
  this->scheduled = scheduled;
}

void Scheduler::tick(time_t time) {
  scheduled(time);
}
