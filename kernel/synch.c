/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * Cosmos AI-OS: Symmetric Multiprocessing Sync (C / Inline ASM Layer)
 * ====================================================================
 * Mission: True Hardware-Level Atomic Locks.
 * Replaces high-level Python locks with native Spinlocks to prevent
 * race conditions inside the kernel scheduler and VFS layers.
 */

#include <stdint.h>

typedef struct {
  volatile uint32_t lock_state; // 0 = Unlocked, 1 = Locked
  uint64_t cpu_id;
} spinlock_t;

void cosmos_spinlock_init(spinlock_t *lock) {
  lock->lock_state = 0;
  lock->cpu_id = 0;
}

// Acquire: Block CPU tightly until the memory is freed
void cosmos_spin_lock(spinlock_t *lock) {
  uint32_t expected;

  while (1) {
    expected = 0;
    // x86_64 Atomic Compare-and-Swap (CMPXCHG)
    // If lock_state == expected (0), set to 1, return success.
    // Lock prefix ensures SMP bus-locking across all CPUs.
    uint8_t result;
    __asm__ volatile("lock cmpxchg %2, %1\n\t"
                     "sete %0"
                     : "=q"(result), "+m"(lock->lock_state)
                     : "r"(1), "a"(expected)
                     : "memory");

    if (result == 1) {
      // Lock Acquired
      break;
    }

    // Spin Loop PAUSE to reduce processor bus overheating
    __asm__ volatile("pause" ::: "memory");
  }
}

// Release: Write 0 to allow waiting CPUs to transition
void cosmos_spin_unlock(spinlock_t *lock) {
  // Memory release fence to ensure previous writes are complete
  __asm__ volatile("sfence" ::: "memory");
  lock->lock_state = 0;
}

