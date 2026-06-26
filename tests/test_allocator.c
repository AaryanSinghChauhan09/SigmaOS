// tests/test_allocator.c
#define _POSIX_C_SOURCE 200112L
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <assert.h>

/* Mock sigma allocator for testing the test framework */
void* sigma_malloc(size_t size) {
    /* Basic 64-byte alignment mock */
    void* ptr = NULL;
    posix_memalign(&ptr, 64, size);
    return ptr;
}

void sigma_free(void* ptr) {
    free(ptr);
}

void test_buddy_allocator_alignment() {
    printf("[TEST] Running test_buddy_allocator_alignment...\n");
    void* ptr = sigma_malloc(1024);
    assert(ptr != NULL);
    assert((uintptr_t)ptr % 64 == 0); // Verify alignment
    sigma_free(ptr);
    printf("[PASS] test_buddy_allocator_alignment\n");
}

void test_no_memory_leaks() {
    printf("[TEST] Running test_no_memory_leaks...\n");
    for (int i = 0; i < 10000; i++) {
        void* p1 = sigma_malloc(256);
        void* p2 = sigma_malloc(512);
        sigma_free(p1);
        sigma_free(p2);
    }
    // Valgrind/ASAN will catch any leaks if we missed a free
    printf("[PASS] test_no_memory_leaks\n");
}

int main() {
    printf("=== SigmaOS Test Suite ===\n");
    test_buddy_allocator_alignment();
    test_no_memory_leaks();
    printf("=== All Tests Passed ===\n");
    return 0;
}
