#include <gtest/gtest.h>
#include "pm_sensor/scheduler.h"

TEST(Scheduler, EmptyConstruction) {
  pm_sensor::Scheduler scheduler;
  ASSERT_EQ(0, scheduler.hourly_hours_mask);
  scheduler.tick(123);
}

TEST(Scheduler, SchedulerHourly) {
  pm_sensor::Scheduler scheduler;
  scheduler.hourly_hours_mask = 0b010011111111111100000000;
  int num_calls = 0;
  scheduler.hourly_callback = [&num_calls] (int32_t current_time) {
    num_calls++;
  };
  // 2019-01-25 01:20:08
  int32_t time_base = 1548379208;

  // 0th hour is excluded by mask
  scheduler.tick(1);
  scheduler.tick(3);
  ASSERT_EQ(0, num_calls);

  // 1st hour is included
  scheduler.tick(time_base);
  ASSERT_EQ(1, num_calls);

  // Should not run again in 1st hour
  scheduler.tick(1);
  scheduler.tick(time_base + 10);
  scheduler.tick(time_base + 60 * 50 + 3);
  ASSERT_EQ(1, num_calls);

  // 4th hour
  scheduler.tick(time_base + 60 * 60 * 3);
  ASSERT_EQ(2, num_calls);
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
