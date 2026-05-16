#ifndef SOVEREIGN_COORDINATION_ZENITH_H
#define SOVEREIGN_COORDINATION_ZENITH_H

#include "./libc/SovereignLibC.h"

#include "./core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Coordination {

// --- ATOMIC HARDWARE INSTRUCTIONS ---
class SovereignAtomicOps {
public:
    static bool TestAndSet(volatile bool* target) {
        bool rv = *target;
        *target = true;
        return rv;
    }
    
    static void Swap(volatile bool* a, volatile bool* b) {
        bool temp = *a;
        *a = *b;
        *b = temp;
    }
};

// --- PROCESS COORDINATION & SYNCHRONIZATION ---
class SovereignPetersonSolution : public SigmaObject {
private:
    volatile bool m_flag[2];
    volatile int m_turn;
public:
    const char* type_name() const noexcept override { return "SovereignPetersonSolution"; }
    void Entering(int i) {
        int j = 1 - i;
        m_flag[i] = true;
        m_turn = j;
        while (m_flag[j] && m_turn == j);
    }
    void Leaving(int i) { m_flag[i] = false; }
};

// --- MONITOR STRUCTURE (SILBERSCHATZ) ---
class SovereignMonitor : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignMonitor"; }
    virtual void InitializationCode() = 0;
    void EnterMonitor() { sigma_printf("[ZENITH-MONITOR]: Mutual exclusion entry.\n"); }
    void LeaveMonitor() { sigma_printf("[ZENITH-MONITOR]: Mutual exclusion release.\n"); }
};

} // namespace Coordination
} // namespace SigmaOS

#endif
