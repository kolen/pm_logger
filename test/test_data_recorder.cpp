#include <gtest/gtest.h>
#include "pm_sensor/data_recorder.h"

struct MyDataPoint {
  int pressure;
  int dow_jones_average;

  bool operator==(const MyDataPoint other) const {
    return pressure == other.pressure && dow_jones_average == other.dow_jones_average;
  }
};

TEST(DataRecorder, Basics) {
  MyDataPoint storage[6];
  pm_sensor::DataRecorder<MyDataPoint> data_recorder(storage, 6, 60);
  int32_t time_base = 23874723;
  MyDataPoint pt = {130, 250};
  data_recorder.add_sample(pt, time_base + 0);
  ASSERT_EQ(pt, data_recorder.get_sample(time_base));
  ASSERT_EQ(MyDataPoint(), data_recorder.get_sample(time_base + 60*6));
  ASSERT_EQ(MyDataPoint(), data_recorder.get_sample(time_base + 60*100));
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
