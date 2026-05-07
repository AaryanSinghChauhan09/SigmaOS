#define SIGMA_HOST
#include <gtest/gtest.h>
#include <string.h>

// A simple mock or helper test
TEST(KernelHelperTest, BasicAssertions) {
    EXPECT_STRNE("hello", "world");
    EXPECT_EQ(7 * 6, 42);
}

int main(int argc, char **argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
