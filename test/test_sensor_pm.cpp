#include <gtest/gtest.h>
#include <gmock/gmock.h>
#include "pm_sensor/sensor_pm.h"

using testing::InSequence;
using testing::Return;
using pm_sensor::PMMeasurement;

class SensorPMDeviceMock : public pm_sensor::SensorPMDevice {
public:
  MOCK_METHOD0(start, void());
  MOCK_METHOD1(setSleepMode, void(bool));
  MOCK_METHOD0(measure, pm_sensor::PMMeasurement());
};

TEST(SensorPM, Init) {
  InSequence is;
  int32_t time_base = 1000000;
  int callback_times = 0;
  PMMeasurement measurement;
  auto callback = [&measurement] (PMMeasurement _measurement) {
    measurement = _measurement;
  };
  SensorPMDeviceMock device;
  pm_sensor::SensorPM sensor(callback, device);
  EXPECT_CALL(device, start());
  sensor.start();

  EXPECT_CALL(device, setSleepMode(false));
  sensor.measure();

  sensor.tick(time_base + 0);
  EXPECT_CALL(device, measure()).WillOnce(Return(PMMeasurement(123, 456)));
  EXPECT_CALL(device, setSleepMode(true));
  sensor.tick(time_base + 9);
  sensor.tick(time_base + 10);
  sensor.tick(time_base + 12);
  sensor.tick(time_base + 20);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
