#include <gtest/gtest.h>

TEST(ExampleSuite, ExampleTest) {
  ASSERT_TRUE(2);
}

int main(int argc, char **argv) {
  ::testing::InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}
