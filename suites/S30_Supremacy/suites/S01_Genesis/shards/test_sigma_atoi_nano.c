/**
 * @file test_sigma_atoi_nano.c
 * @brief Atomic Shard: Functional Unit Test.
 */

int test_sigma_atoi_nano() {
    int res = sigma_atoi("-123");
    if (res == -123) {
        return 0;
    } else {
        return 1;
    }
}
