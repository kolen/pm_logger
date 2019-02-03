#include <gtest/gtest.h>
#include "pm_sensor/data_store.h"

TEST(DataStore, FloatToStored) {
  ASSERT_EQ(172, pm_sensor::DataStore::float_to_stored(17.2));
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
