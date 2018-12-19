#ifdef UNIT_TEST

#include <unity.h>
#include "scheduler.h"

void test_basic_scheduling() {
  Scheduler scheduler;
  bool called = false;
  scheduler.set_hourly([&called](time_t time) {
      TEST_ASSERT_EQUAL(1, time);
      called = true; return true;
    });
  TEST_ASSERT_FALSE(called);
  scheduler.tick((time_t)1);
  TEST_ASSERT_TRUE(called);
}

int main(int argc, char **argv) {
  UNITY_BEGIN();
  RUN_TEST(test_basic_scheduling);
  UNITY_END();
  return 0;
}

#endif
