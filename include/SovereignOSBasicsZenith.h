#include "core/sigma_types.h"
#ifndef SOVEREIGN_OS_BASICS_ZENITH_H
#define SOVEREIGN_OS_BASICS_ZENITH_H

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Basics {

// --- PROCESS CONTROL BLOCK (PCB) & STATES (GEEKSFORGEEKS/Udemy) ---
struct SovereignPCB {
    int pid;
    const char* state; // READY, RUNNING, WAITING, TERMINATED
    sigma_u64 pc;
    sigma_u64 registers[16];
    char* stack_ptr;
};

class SovereignProcessManager : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignProcessManager"; }
    void ContextSwitch(SovereignPCB* old_p, SovereignPCB* new_p);
    void StarvationWatchdog();
};

// --- DEADLOCK HANDLING (BANKER'S ALGORITHM) ---
class SovereignDeadlockAgent : public SigmaObject {
private:
    int m_max[5][3]; // Max resource requirements per process
    int m_allocation[5][3]; // Currently allocated resources
    int m_available[3]; // Total available resources
public:
    const char* type_name() const noexcept override { return "SovereignDeadlockAgent"; }
    bool IsInSafeState();
    void ResourceRequest(int processID, int* request);
};

// --- MEMORY: PAGING & SEGMENTATION (IITB / STANFORD) ---
class SovereignMemoryZenithAdv : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignMemoryZenithAdv"; }
    void Paging(sigma_u64 logicalAddress);
    void Segmentation(sigma_u64 segmentID, sigma_u64 offset);
    void NextFitAllocation();
    void HandleThrashing();
    void PageFaultHandler(sigma_u64 faultingPage);
};

} // namespace Basics
} // namespace SigmaOS

#endif
