#ifndef SOVEREIGN_SYNC_ZENITH_H
#define SOVEREIGN_SYNC_ZENITH_H

#include "./core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Sync {

// --- MUTEX & SEMAPHORE (Hardware Atomic Implementation) ---
class SovereignMutex : public SigmaObject {
private:
    volatile int m_locked;
public:
    SovereignMutex() : m_locked(0) {}
    const char* type_name() const noexcept override { return "SovereignMutex"; }
    
    void Lock() {
        // x86_64 lock xchg logic
        while (__sync_lock_test_and_set(&m_locked, 1));
    }
    
    void Unlock() {
        __sync_lock_release(&m_locked);
    }
};

class SovereignSemaphore : public SigmaObject {
private:
    volatile int m_value;
public:
    SovereignSemaphore(int val) : m_value(val) {}
    const char* type_name() const noexcept override { return "SovereignSemaphore"; }
    
    void Wait() {
        while (m_value <= 0); // Busy wait (Spinlock)
        __sync_fetch_and_sub(&m_value, 1);
    }
    
    void Signal() {
        __sync_fetch_and_add(&m_value, 1);
    }
};

// --- CLASSICAL IPC PROBLEMS (Dinning Philosophers / Readers-Writers) ---
class SovereignSyncProblems : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignSyncProblems"; }
    void SolveDiningPhilosophers();
    void SolveReadersWriters();
};

} // namespace Sync
} // namespace SigmaOS

#endif
