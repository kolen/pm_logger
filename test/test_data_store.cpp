#include <gtest/gtest.h>
#include "pm_sensor/data_store.h"

using pm_sensor::float_to_stored;
using pm_sensor::PMMeasurement;

TEST(DataStore, FloatToStored) {
  uint16_t i;
  for(i=1; i<1000; i++) {
    float f = ((float)i) * 0.1;
    ASSERT_EQ(float_to_stored(f), i)
      << "Expected compact form: " << i << ", passed float: " << f;
  }
}

TEST(DataStore, ConstructPMMeasurement) {
  PMMeasurement m(123.4, 567.8);
  EXPECT_EQ(m.pm2_5, 1234);
  EXPECT_EQ(m.pm10, 5678);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
