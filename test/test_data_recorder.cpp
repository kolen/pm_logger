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
  MyDataPoint pt1 = {0xa1, 0x01};
  MyDataPoint pt2 = {0xa2, 0x02};
  MyDataPoint pt3 = {0xa3, 0x03};
  data_recorder.add_sample(pt1, time_base + 0);

  EXPECT_EQ(pt1, data_recorder.get_at_index(0));
  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_index(1));
  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_index(2));

  data_recorder.add_sample(pt2, time_base + 60);
  data_recorder.add_sample(pt3, time_base + 60 * 2);

  EXPECT_EQ(pt3, data_recorder.get_at_index(0));
  EXPECT_EQ(pt2, data_recorder.get_at_index(1));
  EXPECT_EQ(pt1, data_recorder.get_at_index(2));

  EXPECT_EQ(pt1, data_recorder.get_at_time(time_base));
  EXPECT_EQ(pt2, data_recorder.get_at_time(time_base + 60));
  EXPECT_EQ(pt3, data_recorder.get_at_time(time_base + 60 * 2));

  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_time(time_base + 60*6));
  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_time(time_base + 60*100));

  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_index(6));
  EXPECT_EQ(MyDataPoint(), data_recorder.get_at_index(20));
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
