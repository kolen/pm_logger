#ifdef UNIT_TEST

#include <unity.h>
#include "scheduler.h"

void test_basic_scheduling() {
  Scheduler scheduler;
  int call_times = 0;
  scheduler.set_hourly([&call_times](time_t time) {
      TEST_ASSERT(time != 0);
      call_times++;
      return true;
    });
  TEST_ASSERT_EQUAL(0, call_times);

  struct tm time;
  time_t zero_time = (time_t) 0;
  gmtime_r(&zero_time, &time);
  TEST_ASSERT_EQUAL(0, time.tm_sec);

  time.tm_sec = 10;
  time.tm_min = 15;

  scheduler.tick((time_t)mktime(&time));
  TEST_ASSERT_EQUAL(0, call_times);

  time.tm_min = 0;
  scheduler.tick((time_t)mktime(&time));
  TEST_ASSERT_EQUAL(1, call_times);
  scheduler.tick((time_t)mktime(&time));
  TEST_ASSERT_EQUAL(1, call_times);
}

int main(int argc, char **argv) {
  UNITY_BEGIN();
  RUN_TEST(test_basic_scheduling);
  UNITY_END();
  return 0;
}

#endif
