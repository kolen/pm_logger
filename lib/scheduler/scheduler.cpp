#include "scheduler.h"

void Scheduler::set_hourly(scheduler_callback scheduled) {
  this->scheduled = scheduled;
}

void Scheduler::tick(time_t time) {
  bool success = scheduled(time);
  if (success) {
    last_successful_run = time;
  }
}



bool should_run_at(time_t now, time_t last_successful_run, bool last_is_failure) {

}
