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

TEST(SensorPM, MeasurementSuccess) {
  InSequence is;
  int32_t time_base = 1000000;
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

  EXPECT_CALL(device, measure())
    .WillOnce(Return(PMMeasurement()))
    .WillOnce(Return(PMMeasurement(12.3, 45.6)));
  EXPECT_CALL(device, setSleepMode(true));

  int i;
  for(i=0; i<100; i++) {
    sensor.tick(time_base + i);
  }

  ASSERT_EQ(123, measurement.pm2_5);
  ASSERT_EQ(456, measurement.pm10);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
