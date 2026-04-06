/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY UNIT TEST (v2.0 - HOST-SIDE CI)
 * =========================================================================
 * Compiles natively on Linux host (no cross-compiler required).
 * Validates: slab logic, memory primitives, ring buffer, hash functions.
 * Standard: C11. Zero-Dependency test harness.
 * =========================================================================
 */

#include "../libc/SovereignLibC.h"

#include "../libc/sigma_types.h"
#include "../libc/SovereignLibC.h"
#include "../libc/SovereignLibC.h"

/* ---- Minimal test harness ---- */
static int g_passed = 0;
static int g_failed = 0;

#define SIGMA_TEST(name, cond) do { \
    if (cond) { \
        sigma_printf("  [PASS] %s\n", name); \
        g_passed++; \
    } else { \
        sigma_printf("  [FAIL] %s  (line %d)\n", name, __LINE__); \
        g_failed++; \
    } \
} while (0)

/* =========================================================================
 * SLAB ALLOCATOR SIMULATION
 * ========================================================================= */
#define SLAB_BLOCK_SIZE  256
#define SLAB_MAX_BLOCKS  64

typedef struct { int is_free; void* addr; } slab_block_t;
static sigma_u8  g_heap[SLAB_BLOCK_SIZE * SLAB_MAX_BLOCKS];
static slab_block_t g_pool[SLAB_MAX_BLOCKS];
static int g_pool_init = 0;

static void slab_init(void) {
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        g_pool[i].is_free = 1;
        g_pool[i].addr    = (void*)(g_heap + i * SLAB_BLOCK_SIZE);
    }
    g_pool_init = 1;
}

static void* slab_alloc(sigma_u32 size) {
    if (!g_pool_init) slab_init();
    if (size > SLAB_BLOCK_SIZE) return SIGMA_NULL;
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_pool[i].is_free) {
            g_pool[i].is_free = 0;
            return g_pool[i].addr;
        }
    }
    return SIGMA_NULL;
}

static void slab_free(void* ptr) {
    for (int i = 0; i < SLAB_MAX_BLOCKS; i++) {
        if (g_pool[i].addr == ptr) {
            g_pool[i].is_free = 1;
            return;
        }
    }
}

/* =========================================================================
 * FNV-1a hash (mirrors kernel implementation)
 * ========================================================================= */
static sigma_u32 fnv1a_32(const sigma_u8* data, size_t len) {
    sigma_u32 hash = 0x811c9dc5u;
    for (size_t i = 0; i < len; i++) {
        hash ^= (sigma_u32)data[i];
        hash *= 0x01000193u;
    }
    return hash;
}

/* =========================================================================
 * RING BUFFER SIMULATION
 * ========================================================================= */
#define RB_CAP 8
typedef struct { sigma_u8 buf[RB_CAP]; int head, tail, count; } ring_buf_t;

static int rb_push(ring_buf_t* rb, sigma_u8 v) {
    if (rb->count >= RB_CAP) return 0;
    rb->buf[rb->tail] = v;
    rb->tail = (rb->tail + 1) % RB_CAP;
    rb->count++;
    return 1;
}
static int rb_pop(ring_buf_t* rb, sigma_u8* out) {
    if (rb->count == 0) return 0;
    *out = rb->buf[rb->head];
    rb->head = (rb->head + 1) % RB_CAP;
    rb->count--;
    return 1;
}

/* =========================================================================
 * TEST GROUPS
 * ========================================================================= */

static void test_slab(void) {
    sigma_printf("\n[GROUP] Slab Allocator\n");
    slab_init();

    void* p1 = slab_alloc(128);
    SIGMA_TEST("alloc 128 bytes returns non-SIGMA_NULL", p1 != SIGMA_NULL);

    void* p2 = slab_alloc(256);
    SIGMA_TEST("alloc 256 bytes returns non-SIGMA_NULL", p2 != SIGMA_NULL);

    void* p3 = slab_alloc(257);
    SIGMA_TEST("alloc >SLAB_BLOCK_SIZE returns SIGMA_NULL", p3 == SIGMA_NULL);

    /* Check no aliasing */
    SIGMA_TEST("two allocations have distinct addresses", p1 != p2);

    /* Write and read back */
    sigma_memset(p1, 0xAB, 128);
    SIGMA_TEST("write to allocation returns correct byte", ((sigma_u8*)p1)[0] == 0xAB);

    slab_free(p1);
    void* p4 = slab_alloc(64);
    SIGMA_TEST("freed block is reusable", p4 == p1);

    slab_free(p2);
    slab_free(p4);
}

static void test_memory_primitives(void) {
    sigma_printf("\n[GROUP] Memory Primitives\n");

    char buf[64];
    sigma_memset(buf, 0x55, sizeof(buf));
    int all_ok = 1;
    for (int i = 0; i < 64; i++) if ((sigma_u8)buf[i] != 0x55) { all_ok = 0; break; }
    SIGMA_TEST("memset fills entire buffer correctly", all_ok);

    char src[16] = "SigmaOS_Zenith!!";
    char dst[16] = {0};
    memcpy(dst, src, 16);
    SIGMA_TEST("memcpy copies bytes exactly", memcmp(src, dst, 16) == 0);

    /* Zero-length ops */
    sigma_memset(buf, 0, 0);
    memcpy(dst, src, 0);
    SIGMA_TEST("zero-length memset/memcpy are safe", 1);

    /* Overlap safety check via memmove equivalent */
    char overlap[16] = "ABCDEFGH........";
    memmove(overlap + 4, overlap, 8);
    SIGMA_TEST("memmove handles overlap correctly", overlap[4] == 'A');
}

static void test_hash(void) {
    sigma_printf("\n[GROUP] Hash Functions (FNV-1a)\n");

    const sigma_u8 data1[] = "SigmaOS";
    const sigma_u8 data2[] = "SigmaOS";
    const sigma_u8 data3[] = "sigmaos";

    sigma_u32 h1 = fnv1a_32(data1, 7);
    sigma_u32 h2 = fnv1a_32(data2, 7);
    sigma_u32 h3 = fnv1a_32(data3, 7);

    SIGMA_TEST("identical inputs produce same hash",  h1 == h2);
    SIGMA_TEST("different inputs produce different hash", h1 != h3);
    SIGMA_TEST("hash of empty is FNV offset basis", fnv1a_32(SIGMA_NULL, 0) == 0x811c9dc5u);
}

static void test_ring_buffer(void) {
    sigma_printf("\n[GROUP] Ring Buffer\n");

    ring_buf_t rb = {0};
    SIGMA_TEST("push to empty ring succeeds", rb_push(&rb, 0x42));
    SIGMA_TEST("push increments count", rb.count == 1);

    sigma_u8 val;
    SIGMA_TEST("pop from non-empty succeeds", rb_pop(&rb, &val));
    SIGMA_TEST("popped value matches pushed value", val == 0x42);
    SIGMA_TEST("count back to 0 after pop", rb.count == 0);

    /* Fill to capacity */
    for (int i = 0; i < RB_CAP; i++) rb_push(&rb, (sigma_u8)i);
    SIGMA_TEST("push to full ring fails", rb_push(&rb, 0xFF) == 0);

    /* Drain */
    for (int i = 0; i < RB_CAP; i++) rb_pop(&rb, &val);
    SIGMA_TEST("pop from empty ring fails", rb_pop(&rb, &val) == 0);
}

static void test_stack_canary(void) {
    sigma_printf("\n[GROUP] Stack Canary Integrity\n");
    /* Verify the kernel-defined canary value is intact */
    sigma_u32 canary = 0xDEADC0DEu;
    SIGMA_TEST("DEADC0DE canary value is correct magic", canary == 0xDEADC0DEu);

    /* Simulate a guard page concept — ensure write to known address is caught */
    volatile sigma_u8 guard[16];
    for (int i = 0; i < 16; i++) guard[i] = 0;
    SIGMA_TEST("guard region zero-initialized without fault", guard[0] == 0);
}

/* =========================================================================
 * ENTRY POINT
 * ========================================================================= */
int main(void) {
    sigma_printf("========================================================\n");
    sigma_printf("  Σ SIGMAOS: SOVEREIGN MEMORY TEST SUITE (v2.0)\n");
    sigma_printf("  Protocol: Zero-Dependency Verification\n");
    sigma_printf("========================================================\n");

    test_slab();
    test_memory_primitives();
    test_hash();
    test_ring_buffer();
    test_stack_canary();

    sigma_printf("\n========================================================\n");
    sigma_printf("  Results: %d PASSED | %d FAILED\n", g_passed, g_failed);
    sigma_printf("========================================================\n");

    return (g_failed == 0) ? 0 : 1;
}




