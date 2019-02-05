#include <gtest/gtest.h>
#include "pm_sensor/data_store.h"

TEST(DataStore, FloatToStored) {
  uint16_t i;
  for(i=1; i<1000; i++) {
    float f = ((float)i) * 0.1;
    ASSERT_EQ(pm_sensor::float_to_stored(f), i)
      << "Expected compact form: " << i << ", passed float: " << f;
  }
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
