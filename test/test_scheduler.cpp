#include <gtest/gtest.h>
#include "pm_sensor/scheduler.h"

TEST(Scheduler, EmptyConstruction) {
  pm_sensor::Scheduler scheduler;
  ASSERT_EQ(0, scheduler.hourly_hours_mask);
  scheduler.tick(123);
}

TEST(Scheduler, SchedulingMinutely) {
  pm_sensor::Scheduler scheduler;
  scheduler.minutely_period = 5;
  int num_calls = 0;
  scheduler.minutely_callback = [&num_calls] (int32_t current_time) {
    num_calls++;
  };
  auto time_base = 1514754000;

  scheduler.tick(time_base + 60 * 3);
  ASSERT_EQ(0, num_calls);
  scheduler.tick(time_base + 60 * 5 + 3);
  ASSERT_EQ(1, num_calls);
  scheduler.tick(time_base + 60 * 5 + 59);
  ASSERT_EQ(1, num_calls);
  scheduler.tick(time_base + 60 * 20 + 4);
  ASSERT_EQ(2, num_calls);
  scheduler.tick(time_base + 60 * 15 + 4);
  ASSERT_EQ(2, num_calls);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
