/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

/*
 * SigmaOS Advanced Synchronization Primitives
 * ==========================================
 * Object-Oriented Synchronization with SOLID Principles
 */

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>

// Atomic operations (x86_64)
static inline uint32_t atomic_load(volatile uint32_t* ptr) {
    uint32_t value;
    __asm__ volatile ("movl %1, %0" : "=r"(value) : "m"(*ptr));
    return value;
}

static inline void atomic_store(volatile uint32_t* ptr, uint32_t value) {
    __asm__ volatile ("movl %1, %0" : "=m"(*ptr) : "r"(value));
}

static inline uint32_t atomic_exchange(volatile uint32_t* ptr, uint32_t value) {
    uint32_t result;
    __asm__ volatile ("xchgl %1, %0" : "=r"(result), "+m"(*ptr) : "0"(value));
    return result;
}

static inline bool atomic_compare_exchange(volatile uint32_t* ptr, uint32_t* expected, uint32_t desired) {
    uint32_t result;
    bool success;
    __asm__ volatile ("lock; cmpxchgl %2, %1"
                      : "=a"(result), "+m"(*ptr), "+r"(*expected)
                      : "r"(desired)
                      : "memory");
    success = (result == *expected);
    return success;
}

static inline uint32_t atomic_fetch_add(volatile uint32_t* ptr, uint32_t value) {
    uint32_t result;
    __asm__ volatile ("lock; xaddl %1, %0"
                      : "=r"(result), "+m"(*ptr)
                      : "0"(value)
                      : "memory");
    return result;
}

static inline uint32_t atomic_fetch_sub(volatile uint32_t* ptr, uint32_t value) {
    return atomic_fetch_add(ptr, -value);
}

// Memory ordering
typedef enum {
    MEMORY_ORDER_RELAXED = 0,
    MEMORY_ORDER_ACQUIRE = 1,
    MEMORY_ORDER_RELEASE = 2,
    MEMORY_ORDER_ACQ_REL = 3,
    MEMORY_ORDER_SEQ_CST = 4
} MemoryOrder;

// Memory barriers
static inline void memory_barrier(void) {
    __asm__ volatile ("mfence" ::: "memory");
}

static inline void acquire_barrier(void) {
    __asm__ volatile ("lfence" ::: "memory");
}

static inline void release_barrier(void) {
    __asm__ volatile ("sfence" ::: "memory");
}

// OOP: Synchronization primitive interface
typedef struct SyncPrimitive SyncPrimitive;
typedef struct SyncManager SyncManager;

// Wait queue structure
typedef struct WaitQueue {
    uint32_t waiters;
    SyncPrimitive* primitive;
    struct WaitQueue* next;
} WaitQueue;

// Thread control block (simplified)
typedef struct {
    uint32_t tid;
    uint32_t priority;
    enum {
        THREAD_STATE_RUNNING,
        THREAD_STATE_READY,
        THREAD_STATE_BLOCKED,
        THREAD_STATE_WAITING
    } state;
    WaitQueue* wait_queue;
    struct Thread* next;
} Thread;

// Synchronization primitive base class
struct SyncPrimitive {
    uint32_t id;
    char name[32];
    enum {
        SYNC_TYPE_MUTEX,
        SYNC_TYPE_SEMAPHORE,
        SYNC_TYPE_CONDITION,
        SYNC_TYPE_RWLOCK,
        SYNC_TYPE_BARRIER,
        SYNC_TYPE_SPINLOCK
    } type;
    volatile uint32_t state;
    WaitQueue* wait_queue;
    uint32_t owner_tid;
    uint32_t ref_count;
    void (*destroy)(SyncPrimitive* primitive);
    struct SyncPrimitive* next;
};

// Mutex implementation
typedef struct {
    SyncPrimitive base;
    volatile uint32_t lock_count;
    uint32_t recursion_count;
    Thread* owner;
    Thread* waiters;
} Mutex;

// Semaphore implementation
typedef struct {
    SyncPrimitive base;
    volatile uint32_t count;
    uint32_t max_count;
    Thread* waiters;
} Semaphore;

// Condition variable implementation
typedef struct {
    SyncPrimitive base;
    Thread* waiters;
    Mutex* associated_mutex;
} Condition;

// Read-write lock implementation
typedef struct {
    SyncPrimitive base;
    volatile uint32_t state; // Bits: readers in low 16, writer bit in high 16
    Thread* writer_waiters;
    Thread* reader_waiters;
    uint32_t writer_count;
} RWLock;

// Barrier implementation
typedef struct {
    SyncPrimitive base;
    volatile uint32_t count;
    volatile uint32_t waiting;
    uint32_t threshold;
    Thread* waiters;
} Barrier;

// Spinlock implementation
typedef struct {
    SyncPrimitive base;
    volatile uint32_t lock;
} Spinlock;

// Synchronization Manager
struct SyncManager {
    SyncPrimitive* primitives[256];
    SyncPrimitive* primitive_list;
    uint32_t next_primitive_id;
    
    // Statistics
    uint64_t total_locks;
    uint64_t total_unlocks;
    uint64_t contentions;
    uint64_t timeouts;
    
    // Configuration
    uint32_t spin_threshold;
    uint32_t max_wait_time;
    bool deadlock_detection_enabled;
};

// OOP: Mutex operations
static void mutex_lock(Mutex* mutex);
static void mutex_unlock(Mutex* mutex);
static bool mutex_trylock(Mutex* mutex);
static void mutex_destroy(SyncPrimitive* primitive);

// OOP: Semaphore operations
static void semaphore_wait(Semaphore* sem);
static void semaphore_signal(Semaphore* sem);
static bool semaphore_trywait(Semaphore* sem);
static void semaphore_destroy(SyncPrimitive* primitive);

// OOP: Condition variable operations
static void condition_wait(Condition* cond, Mutex* mutex);
static void condition_signal(Condition* cond);
static void condition_broadcast(Condition* cond);
static void condition_destroy(SyncPrimitive* primitive);

// OOP: Read-write lock operations
static void rwlock_read_lock(RWLock* rwlock);
static void rwlock_write_lock(RWLock* rwlock);
static void rwlock_read_unlock(RWLock* rwlock);
static void rwlock_write_unlock(RWLock* rwlock);
static bool rwlock_try_read_lock(RWLock* rwlock);
static bool rwlock_try_write_lock(RWLock* rwlock);
static void rwlock_destroy(SyncPrimitive* primitive);

// OOP: Barrier operations
static void barrier_wait(Barrier* barrier);
static void barrier_destroy(SyncPrimitive* primitive);

// OOP: Spinlock operations
static void spinlock_lock(Spinlock* spinlock);
static void spinlock_unlock(Spinlock* spinlock);
static bool spinlock_trylock(Spinlock* spinlock);
static void spinlock_destroy(SyncPrimitive* primitive);

// Thread operations (simplified)
static void thread_block(Thread* thread);
static void thread_unblock(Thread* thread);
static void thread_yield(void);

// Mutex implementation
static void mutex_lock(Mutex* mutex) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Fast path: try to acquire lock
    if (atomic_exchange(&mutex->base.state, 1) == 0) {
        mutex->owner = current_thread;
        mutex->lock_count = 1;
        mutex->recursion_count = 1;
        return;
    }
    
    // Check for recursive lock
    if (mutex->owner == current_thread) {
        mutex->recursion_count++;
        mutex->lock_count++;
        return;
    }
    
    // Slow path: block and wait
    do {
        // Add to waiters list
        current_thread->state = THREAD_STATE_WAITING;
        current_thread->wait_queue = (WaitQueue*)mutex;
        
        // Add to waiters list
        current_thread->next = mutex->waiters;
        mutex->waiters = current_thread;
        
        // Block the thread
        thread_block(current_thread);
        
        // When woken, try to acquire lock
    } while (atomic_exchange(&mutex->base.state, 1) != 0);
    
    // Acquired lock
    mutex->owner = current_thread;
    mutex->lock_count = 1;
    mutex->recursion_count = 1;
}

static void mutex_unlock(Mutex* mutex) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Check if current thread owns the mutex
    if (mutex->owner != current_thread) {
        return; // Error: unlocking mutex not owned by current thread
    }
    
    mutex->recursion_count--;
    mutex->lock_count--;
    
    if (mutex->recursion_count > 0) {
        return; // Still have recursive locks
    }
    
    // Release the lock
    mutex->owner = NULL;
    atomic_store(&mutex->base.state, 0);
    
    // Wake up next waiter
    if (mutex->waiters) {
        Thread* next = mutex->waiters;
        mutex->waiters = next->next;
        next->state = THREAD_STATE_READY;
        next->wait_queue = NULL;
        thread_unblock(next);
    }
}

static bool mutex_trylock(Mutex* mutex) {
    Thread* current_thread = sigma_get_current_thread();
    
    if (atomic_exchange(&mutex->base.state, 1) == 0) {
        mutex->owner = current_thread;
        mutex->lock_count = 1;
        mutex->recursion_count = 1;
        return true;
    }
    
    if (mutex->owner == current_thread) {
        mutex->recursion_count++;
        mutex->lock_count++;
        return true;
    }
    
    return false;
}

static void mutex_destroy(SyncPrimitive* primitive) {
    Mutex* mutex = (Mutex*)primitive;
    if (mutex->waiters) {
        // Wake up all waiters
        Thread* waiter = mutex->waiters;
        while (waiter) {
            Thread* next = waiter->next;
            waiter->state = THREAD_STATE_READY;
            waiter->wait_queue = NULL;
            thread_unblock(waiter);
            waiter = next;
        }
    }
    free(mutex);
}

// Semaphore implementation
static void semaphore_wait(Semaphore* sem) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Fast path: try to acquire
    if (atomic_fetch_sub(&sem->count, 1) > 0) {
        return;
    }
    
    // Slow path: block and wait
    do {
        current_thread->state = THREAD_STATE_WAITING;
        current_thread->wait_queue = (WaitQueue*)sem;
        
        // Add to waiters list
        current_thread->next = sem->waiters;
        sem->waiters = current_thread;
        
        // Block the thread
        thread_block(current_thread);
        
        // When woken, try to acquire
    } while (atomic_fetch_sub(&sem->count, 1) == 0);
}

static void semaphore_signal(Semaphore* sem) {
    // Increment count
    atomic_fetch_add(&sem->count, 1);
    
    // Wake up one waiter
    if (sem->waiters) {
        Thread* next = sem->waiters;
        sem->waiters = next->next;
        next->state = THREAD_STATE_READY;
        next->wait_queue = NULL;
        thread_unblock(next);
    }
}

static bool semaphore_trywait(Semaphore* sem) {
    uint32_t old_count = atomic_load(&sem->count);
    while (old_count > 0) {
        if (atomic_compare_exchange(&sem->count, &old_count, old_count - 1)) {
            return true;
        }
    }
    return false;
}

static void semaphore_destroy(SyncPrimitive* primitive) {
    Semaphore* sem = (Semaphore*)primitive;
    if (sem->waiters) {
        // Wake up all waiters
        Thread* waiter = sem->waiters;
        while (waiter) {
            Thread* next = waiter->next;
            waiter->state = THREAD_STATE_READY;
            waiter->wait_queue = NULL;
            thread_unblock(waiter);
            waiter = next;
        }
    }
    free(sem);
}

// Condition variable implementation
static void condition_wait(Condition* cond, Mutex* mutex) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Add to waiters list
    current_thread->state = THREAD_STATE_WAITING;
    current_thread->wait_queue = (WaitQueue*)cond;
    
    current_thread->next = cond->waiters;
    cond->waiters = current_thread;
    
    // Store associated mutex
    cond->associated_mutex = mutex;
    
    // Release mutex and block
    uint32_t recursion_count = mutex->recursion_count;
    mutex_unlock(mutex);
    
    thread_block(current_thread);
    
    // When woken, re-acquire mutex
    mutex_lock(mutex);
    mutex->recursion_count = recursion_count;
}

static void condition_signal(Condition* cond) {
    if (cond->waiters) {
        Thread* next = cond->waiters;
        cond->waiters = next->next;
        next->state = THREAD_STATE_READY;
        next->wait_queue = NULL;
        thread_unblock(next);
    }
}

static void condition_broadcast(Condition* cond) {
    Thread* waiter = cond->waiters;
    while (waiter) {
        Thread* next = waiter->next;
        waiter->state = THREAD_STATE_READY;
        waiter->wait_queue = NULL;
        thread_unblock(waiter);
        waiter = next;
    }
    cond->waiters = NULL;
}

static void condition_destroy(SyncPrimitive* primitive) {
    Condition* cond = (Condition*)primitive;
    if (cond->waiters) {
        // Wake up all waiters
        Thread* waiter = cond->waiters;
        while (waiter) {
            Thread* next = waiter->next;
            waiter->state = THREAD_STATE_READY;
            waiter->wait_queue = NULL;
            thread_unblock(waiter);
            waiter = next;
        }
    }
    free(cond);
}

// Read-write lock implementation
static void rwlock_read_lock(RWLock* rwlock) {
    Thread* current_thread = sigma_get_current_thread();
    
    while (true) {
        uint32_t state = atomic_load(&rwlock->state);
        
        // Check if we can acquire read lock (no writer)
        if ((state & 0xFFFF0000) == 0) {
            uint32_t new_state = state + 1;
            if (atomic_compare_exchange(&rwlock->state, &state, new_state)) {
                return;
            }
        } else {
            // Writer present, wait
            current_thread->state = THREAD_STATE_WAITING;
            current_thread->wait_queue = (WaitQueue*)rwlock;
            
            current_thread->next = rwlock->reader_waiters;
            rwlock->reader_waiters = current_thread;
            
            thread_block(current_thread);
        }
    }
}

static void rwlock_write_lock(RWLock* rwlock) {
    Thread* current_thread = sigma_get_current_thread();
    
    while (true) {
        uint32_t state = atomic_load(&rwlock->state);
        
        // Check if we can acquire write lock (no readers or writers)
        if (state == 0) {
            uint32_t new_state = 0x00010000; // Set writer bit
            if (atomic_compare_exchange(&rwlock->state, &state, new_state)) {
                rwlock->writer_count = 1;
                return;
            }
        } else if ((state & 0x00010000) && rwlock->writer_count > 0) {
            // Already have write lock, increment count
            uint32_t new_state = state + 0x00010000;
            if (atomic_compare_exchange(&rwlock->state, &state, new_state)) {
                rwlock->writer_count++;
                return;
            }
        } else {
            // Readers or other writers present, wait
            current_thread->state = THREAD_STATE_WAITING;
            current_thread->wait_queue = (WaitQueue*)rwlock;
            
            current_thread->next = rwlock->writer_waiters;
            rwlock->writer_waiters = current_thread;
            
            thread_block(current_thread);
        }
    }
}

static void rwlock_read_unlock(RWLock* rwlock) {
    uint32_t old_state = atomic_fetch_sub(&rwlock->state, 1);
    
    // If this was the last reader, wake up writers
    if ((old_state & 0xFFFF) == 1) {
        if (rwlock->writer_waiters) {
            Thread* next = rwlock->writer_waiters;
            rwlock->writer_waiters = next->next;
            next->state = THREAD_STATE_READY;
            next->wait_queue = NULL;
            thread_unblock(next);
        }
    }
}

static void rwlock_write_unlock(RWLock* rwlock) {
    rwlock->writer_count--;
    if (rwlock->writer_count > 0) {
        atomic_fetch_sub(&rwlock->state, 0x00010000);
        return;
    }
    
    uint32_t old_state = atomic_fetch_sub(&rwlock->state, 0x00010000);
    
    // Wake up readers first, then writers
    if (rwlock->reader_waiters) {
        Thread* waiter = rwlock->reader_waiters;
        while (waiter) {
            Thread* next = waiter->next;
            waiter->state = THREAD_STATE_READY;
            waiter->wait_queue = NULL;
            thread_unblock(waiter);
            waiter = next;
        }
        rwlock->reader_waiters = NULL;
    } else if (rwlock->writer_waiters) {
        Thread* next = rwlock->writer_waiters;
        rwlock->writer_waiters = next->next;
        next->state = THREAD_STATE_READY;
        next->wait_queue = NULL;
        thread_unblock(next);
    }
}

static bool rwlock_try_read_lock(RWLock* rwlock) {
    uint32_t state = atomic_load(&rwlock->state);
    
    if ((state & 0xFFFF0000) == 0) {
        uint32_t new_state = state + 1;
        return atomic_compare_exchange(&rwlock->state, &state, new_state);
    }
    
    return false;
}

static bool rwlock_try_write_lock(RWLock* rwlock) {
    uint32_t state = atomic_load(&rwlock->state);
    
    if (state == 0) {
        uint32_t new_state = 0x00010000;
        if (atomic_compare_exchange(&rwlock->state, &state, new_state)) {
            rwlock->writer_count = 1;
            return true;
        }
    }
    
    return false;
}

static void rwlock_destroy(SyncPrimitive* primitive) {
    RWLock* rwlock = (RWLock*)primitive;
    
    // Wake up all waiters
    Thread* waiter = rwlock->reader_waiters;
    while (waiter) {
        Thread* next = waiter->next;
        waiter->state = THREAD_STATE_READY;
        waiter->wait_queue = NULL;
        thread_unblock(waiter);
        waiter = next;
    }
    
    waiter = rwlock->writer_waiters;
    while (waiter) {
        Thread* next = waiter->next;
        waiter->state = THREAD_STATE_READY;
        waiter->wait_queue = NULL;
        thread_unblock(waiter);
        waiter = next;
    }
    
    free(rwlock);
}

// Spinlock implementation
static void spinlock_lock(Spinlock* spinlock) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Spin until we acquire the lock
    while (atomic_exchange(&spinlock->lock, 1) != 0) {
        // Yield CPU to other threads
        thread_yield();
    }
}

static void spinlock_unlock(Spinlock* spinlock) {
    atomic_store(&spinlock->lock, 0);
}

static bool spinlock_trylock(Spinlock* spinlock) {
    return atomic_exchange(&spinlock->lock, 1) == 0;
}

static void spinlock_destroy(SyncPrimitive* primitive) {
    Spinlock* spinlock = (Spinlock*)primitive;
    free(spinlock);
}

// Barrier implementation
static void barrier_wait(Barrier* barrier) {
    Thread* current_thread = sigma_get_current_thread();
    
    // Add to waiters
    current_thread->state = THREAD_STATE_WAITING;
    current_thread->wait_queue = (WaitQueue*)barrier;
    
    current_thread->next = barrier->waiters;
    barrier->waiters = current_thread;
    
    uint32_t waiting = atomic_fetch_add(&barrier->waiting, 1) + 1;
    
    if (waiting == barrier->threshold) {
        // Last thread to arrive, wake everyone
        Thread* waiter = barrier->waiters;
        while (waiter) {
            Thread* next = waiter->next;
            waiter->state = THREAD_STATE_READY;
            waiter->wait_queue = NULL;
            thread_unblock(waiter);
            waiter = next;
        }
        barrier->waiters = NULL;
        atomic_store(&barrier->waiting, 0);
    } else {
        // Block until all threads arrive
        thread_block(current_thread);
    }
}

static void barrier_destroy(SyncPrimitive* primitive) {
    Barrier* barrier = (Barrier*)primitive;
    
    // Wake up all waiters
    Thread* waiter = barrier->waiters;
    while (waiter) {
        Thread* next = waiter->next;
        waiter->state = THREAD_STATE_READY;
        waiter->wait_queue = NULL;
        thread_unblock(waiter);
        waiter = next;
    }
    
    free(barrier);
}

// Synchronization Manager Constructor
SyncManager* sigma_sync_manager_create(void) {
    SyncManager* manager = (SyncManager*)malloc(sizeof(SyncManager));
    if (!manager) return NULL;
    
    memset(manager, 0, sizeof(SyncManager));
    manager->next_primitive_id = 1;
    manager->spin_threshold = 1000;
    manager->max_wait_time = 5000;
    manager->deadlock_detection_enabled = true;
    
    return manager;
}

// Create mutex (Factory Method)
uint32_t sigma_sync_create_mutex(SyncManager* manager, const char* name) {
    Mutex* mutex = (Mutex*)malloc(sizeof(Mutex));
    if (!mutex) return 0;
    
    mutex->base.id = manager->next_primitive_id++;
    strncpy(mutex->base.name, name, sizeof(mutex->base.name) - 1);
    mutex->base.type = SYNC_TYPE_MUTEX;
    mutex->base.state = 0;
    mutex->base.wait_queue = NULL;
    mutex->base.owner_tid = 0;
    mutex->base.ref_count = 1;
    mutex->base.destroy = mutex_destroy;
    
    mutex->lock_count = 0;
    mutex->recursion_count = 0;
    mutex->owner = NULL;
    mutex->waiters = NULL;
    
    // Add to manager
    mutex->base.next = manager->primitive_list;
    manager->primitive_list = (SyncPrimitive*)mutex;
    
    if (mutex->base.id < 256) {
        manager->primitives[mutex->base.id] = (SyncPrimitive*)mutex;
    }
    
    return mutex->base.id;
}

// Create semaphore (Factory Method)
uint32_t sigma_sync_create_semaphore(SyncManager* manager, const char* name, uint32_t initial_count, uint32_t max_count) {
    Semaphore* sem = (Semaphore*)malloc(sizeof(Semaphore));
    if (!sem) return 0;
    
    sem->base.id = manager->next_primitive_id++;
    strncpy(sem->base.name, name, sizeof(sem->base.name) - 1);
    sem->base.type = SYNC_TYPE_SEMAPHORE;
    sem->base.state = 0;
    sem->base.wait_queue = NULL;
    sem->base.owner_tid = 0;
    sem->base.ref_count = 1;
    sem->base.destroy = semaphore_destroy;
    
    sem->count = initial_count;
    sem->max_count = max_count;
    sem->waiters = NULL;
    
    // Add to manager
    sem->base.next = manager->primitive_list;
    manager->primitive_list = (SyncPrimitive*)sem;
    
    if (sem->base.id < 256) {
        manager->primitives[sem->base.id] = (SyncPrimitive*)sem;
    }
    
    return sem->base.id;
}

// Create condition variable (Factory Method)
uint32_t sigma_sync_create_condition(SyncManager* manager, const char* name) {
    Condition* cond = (Condition*)malloc(sizeof(Condition));
    if (!cond) return 0;
    
    cond->base.id = manager->next_primitive_id++;
    strncpy(cond->base.name, name, sizeof(cond->base.name) - 1);
    cond->base.type = SYNC_TYPE_CONDITION;
    cond->base.state = 0;
    cond->base.wait_queue = NULL;
    cond->base.owner_tid = 0;
    cond->base.ref_count = 1;
    cond->base.destroy = condition_destroy;
    
    cond->waiters = NULL;
    cond->associated_mutex = NULL;
    
    // Add to manager
    cond->base.next = manager->primitive_list;
    manager->primitive_list = (SyncPrimitive*)cond;
    
    if (cond->base.id < 256) {
        manager->primitives[cond->base.id] = (SyncPrimitive*)cond;
    }
    
    return cond->base.id;
}

// Get synchronization primitive by ID
SyncPrimitive* sigma_sync_get_primitive(SyncManager* manager, uint32_t id) {
    if (!manager || id == 0) return NULL;
    
    if (id < 256) {
        return manager->primitives[id];
    }
    
    // Search in list
    SyncPrimitive* primitive = manager->primitive_list;
    while (primitive) {
        if (primitive->id == id) {
            return primitive;
        }
        primitive = primitive->next;
    }
    
    return NULL;
}

// Thread operations (simplified - would interface with scheduler)
static Thread* sigma_get_current_thread(void) {
    static Thread dummy_thread = {.tid = 1, .priority = 0, .state = THREAD_STATE_RUNNING};
    return &dummy_thread;
}

static void thread_block(Thread* thread) {
    thread->state = THREAD_STATE_BLOCKED;
    // This would interface with the scheduler
}

static void thread_unblock(Thread* thread) {
    thread->state = THREAD_STATE_READY;
    // This would interface with the scheduler
}

static void thread_yield(void) {
    // This would interface with the scheduler
}

// Synchronization Manager Destructor
void sigma_sync_manager_destroy(SyncManager* manager) {
    if (!manager) return;
    
    // Destroy all primitives
    SyncPrimitive* primitive = manager->primitive_list;
    while (primitive) {
        SyncPrimitive* next = primitive->next;
        if (primitive->destroy) {
            primitive->destroy(primitive);
        }
        primitive = next;
    }
    
    free(manager);
}

