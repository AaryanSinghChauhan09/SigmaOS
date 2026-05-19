/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: CRYPTOGRAPHIC ENTROPY POOL (/dev/urandom)
 * =============================================================================
 * Inspired by: Linux kernel drivers/char/random.c
 *              FreeBSD sys/dev/random/randomdev.c
 * =============================================================================
 * Collects environmental noise and hardware entropy to seed a PRNG.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define ENTROPY_POOL_SIZE 512

typedef struct {
    sigma_u32 pool[ENTROPY_POOL_SIZE / 4];
    sigma_u32 input_rotate;
    sigma_u32 add_ptr;
    sigma_u32 entropy_count; /* Bits of entropy estimated */
} sigma_entropy_t;

static sigma_entropy_t random_state;

/* Fast unsecure mixer (simulating ChaCha20/SHA mixer in real kernel) */
static void mix_pool_bytes(const sigma_u8* bytes, sigma_u32 len) {
    for (sigma_u32 i = 0; i < len; i++) {
        random_state.pool[random_state.add_ptr] ^= (sigma_u32)bytes[i] << random_state.input_rotate;
        random_state.input_rotate = (random_state.input_rotate + 7) % 24;
        
        random_state.add_ptr = (random_state.add_ptr + 1) % (ENTROPY_POOL_SIZE / 4);
    }
}

void random_init(void) {
    sigma_memset(&random_state, 0, sizeof(random_state));
    sigma_printf("[random] Entropy pool initialized (%d bytes)\n", ENTROPY_POOL_SIZE);
}

void random_add_entropy(const void* data, sigma_u32 len, sigma_u32 bits) {
    mix_pool_bytes((const sigma_u8*)data, len);
    
    random_state.entropy_count += bits;
    if (random_state.entropy_count > ENTROPY_POOL_SIZE * 8) {
        random_state.entropy_count = ENTROPY_POOL_SIZE * 8;
    }
    
    sigma_printf("[random] Added %u bits of entropy. Pool estimate: %u bits\n", 
                 bits, random_state.entropy_count);
}

/* Hardware interrupt entropy collector (simulated) */
void random_add_interrupt_entropy(sigma_u32 irq_num, sigma_u64 tsc) {
    sigma_u32 entropy_data[3] = { irq_num, (sigma_u32)tsc, (sigma_u32)(tsc >> 32) };
    /* Assume 1 bit of true unpredictability per interrupt timing */
    random_add_entropy(entropy_data, sizeof(entropy_data), 1);
}

/* Retrieve random bytes (simulating extraction from pool) */
void get_random_bytes(void* buf, sigma_u32 len) {
    sigma_u8* out = (sigma_u8*)buf;
    
    /* In a real implementation, this would use a CSPRNG like ChaCha20 seeded by the pool.
       We simulate extraction by mixing and taking bytes directly. */
       
    for (sigma_u32 i = 0; i < len; i++) {
        /* Terrible PRNG for simulation purposes only */
        random_state.pool[0] = (random_state.pool[0] * 1103515245 + 12345) & 0x7FFFFFFF;
        out[i] = (sigma_u8)(random_state.pool[0] ^ random_state.pool[random_state.add_ptr]);
        
        random_state.add_ptr = (random_state.add_ptr + 1) % (ENTROPY_POOL_SIZE / 4);
    }
    
    if (random_state.entropy_count >= len * 8) {
        random_state.entropy_count -= len * 8;
    } else {
        random_state.entropy_count = 0;
    }
}

sigma_u32 get_random_u32(void) {
    sigma_u32 val;
    get_random_bytes(&val, sizeof(val));
    return val;
}
