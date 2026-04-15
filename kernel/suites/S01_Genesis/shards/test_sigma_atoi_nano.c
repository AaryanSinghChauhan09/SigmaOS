/**
 * @file test_sigma_atoi_nano.c
 * @brief Atomic Shard: Functional Unit Test.
 */

#include "suites/S01_Genesis/shards/sigma_libc.h"

int main() {
    int res = sigma_atoi("-123");
    if (res == -123) {
        sigma_print("S [TEST]: sigma_atoi PASSED.\n");
        return 0;
    } else {
        sigma_print("S [TEST]: sigma_atoi FAILED.\n");
        return 1;
    }
}
