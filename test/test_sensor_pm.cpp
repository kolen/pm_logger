#include <gtest/gtest.h>
#include "pm_sensor/sensor_pm.h"

TEST(SensorPM, Init) {
  int callback_times = 0;
  pm_sensor::PMMeasurement measurement;
  auto callback = [&measurement] (pm_sensor::PMMeasurement _measurement) {
    measurement = _measurement;
  };
  pm_sensor::SensorPM sensor(callback);
  ASSERT_EQ(callback_times, 0);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
