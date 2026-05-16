#include "../../../include/sigma_types.h"
#ifndef SOVEREIGN_DISK_ZENITH_H
#define SOVEREIGN_DISK_ZENITH_H

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Disk {

// --- DISK SCHEDULING ALGORITHMS (SSTF, SCAN, LOOK) ---
class SovereignDiskScheduler : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignDiskScheduler"; }
    void SSTF_Schedule(int* requests, int count, int initialHead);
    void SCAN_Schedule(int* requests, int count, int initialHead);
    void SSTF_Program_Sim();
};

// --- FILE SYSTEM STRUCTURES (Contiguous, Linked, Indexed) ---
class SovereignFileSystemShard : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignFileSystemShard"; }
    void ContiguousAllocation();
    void LinkedAllocation();
    void IndexedAllocation();
    void FreeSpaceManagement(sigma_u64 freeBlocksBitVector);
};

// --- SPOOLING VS BUFFERING (I/O) ---
class SovereignIOExpert : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignIOExpert"; }
    void SpoolingDaemon();
    void BufferingLogic();
};

} // namespace Disk
} // namespace SigmaOS

#endif
